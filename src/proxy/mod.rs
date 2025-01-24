mod static_files;
mod cors;

use axum::{
    Router,
    routing::any,
    http::{Request, Response, StatusCode},
    body::{Body, to_bytes},
};
use hyper_util::{
    client::legacy::{Client, Builder},
    rt::TokioExecutor,
};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use http_body_util::{BodyExt, combinators::BoxBody};
use std::sync::Arc;
use std::path::PathBuf;
use bytes::Bytes;
use tracing::info;
use chrono::Utc;
use crate::config::Config;
use crate::logger::{Logger, RequestLog};
use self::static_files::static_files_service;
use self::cors::create_cors_layer;

type BoxHttpBody = BoxBody<Bytes, hyper::Error>;

pub struct ProxyServer {
    client: Client<HttpsConnector<HttpConnector>, BoxHttpBody>,
    config: Arc<Config>,
    logger: Arc<Logger>,
}

impl ProxyServer {
    pub fn new(config: Config, logger: Arc<Logger>) -> Self {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        let https = HttpsConnector::new_with_connector(http);
        
        let client = Builder::new(TokioExecutor::new())
            .build(https);
        
        ProxyServer {
            client,
            config: Arc::new(config),
            logger,
        }
    }

    pub fn router(self) -> Router {
        let mut router = Router::new();
        let config = self.config.clone();

        // Добавляем обработку статических файлов, если она включена
        if config.static_files.enabled {
            let static_dir = PathBuf::from(&config.static_files.directory);
            router = router.merge(static_files_service(static_dir));
        }

        // Добавляем CORS middleware
        router = router.layer(create_cors_layer(&config.cors));

        // Добавляем маршруты логгера
        router = router.merge(crate::logger::api::create_logger_routes(self.logger.clone()));

        // Добавляем прокси-обработчик для каждого правила
        for rule in &config.proxy.rules {
            let path = format!("{}/*rest", rule.path);
            let client = self.client.clone();
            let config = self.config.clone();
            let logger = self.logger.clone();
            router = router.route(&path, any(move |req: Request<Body>| {
                let client = client.clone();
                let config = config.clone();
                let logger = logger.clone();
                async move {
                    let start_time = Utc::now();
                    let method = req.method().to_string();
                    let path = req.uri().path().to_string();
                    let request_headers = req.headers()
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                        .collect();

                    match handle_proxy_request(client, config, req).await {
                        Ok(response) => {
                            let duration = (Utc::now() - start_time).num_milliseconds() as u64;
                            let status = response.status().as_u16();
                            let response_headers = response.headers()
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                                .collect();

                            // Создаем запись в логе
                            let log = RequestLog {
                                timestamp: start_time,
                                method,
                                path: path.clone(),
                                target_url: response.headers()
                                    .get("x-proxy-target")
                                    .and_then(|v| v.to_str().ok())
                                    .unwrap_or("")
                                    .to_string(),
                                status,
                                duration_ms: duration,
                                request_headers,
                                response_headers,
                            };

                            // Асинхронно добавляем запись в лог
                            let logger = logger.clone();
                            tokio::spawn(async move {
                                logger.add_log(log).await;
                            });

                            response
                        }
                        Err(e) => {
                            let duration = (Utc::now() - start_time).num_milliseconds() as u64;
                            let error_msg = e.to_string();
                            
                            // Создаем запись в логе для ошибки
                            let log = RequestLog {
                                timestamp: start_time,
                                method,
                                path,
                                target_url: "".to_string(),
                                status: StatusCode::BAD_GATEWAY.as_u16(),
                                duration_ms: duration,
                                request_headers,
                                response_headers: vec![],
                            };

                            // Асинхронно добавляем запись в лог
                            let logger = logger.clone();
                            tokio::spawn(async move {
                                logger.add_log(log).await;
                            });

                            info!("Proxy error: {}", error_msg);
                            Response::builder()
                                .status(StatusCode::BAD_GATEWAY)
                                .body(Body::from(format!("Proxy error: {}", error_msg)))
                                .unwrap()
                        }
                    }
                }
            }));
        }

        router
    }
}

async fn handle_proxy_request(
    client: Client<HttpsConnector<HttpConnector>, BoxHttpBody>,
    config: Arc<Config>,
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    // Находим подходящее правило проксирования
    let path = req.uri().path();
    let rule = config.proxy.rules.iter()
        .find(|r| path.starts_with(&r.path))
        .ok_or("No matching proxy rule found")?;

    // Создаем новый URI для проксированного запроса
    let new_path = path.replacen(&rule.path, "", 1);
    let target = rule.target.trim_end_matches('/');
    let new_uri = if new_path.starts_with('/') {
        format!("{}{}", target, new_path)
    } else {
        format!("{}/{}", target, new_path)
    };

    info!("Proxying request: {} -> {}", path, new_uri);

    // Создаем новый запрос
    let (parts, body) = req.into_parts();
    let mut new_req = Request::builder()
        .uri(new_uri.clone())
        .method(parts.method);

    // Копируем заголовки
    for (name, value) in parts.headers.iter() {
        if name != hyper::header::HOST {
            new_req = new_req.header(name, value);
        }
    }

    // Добавляем User-Agent для GitHub API
    new_req = new_req.header(hyper::header::USER_AGENT, "useproxy/1.0");

    // Собираем тело запроса в байты
    let body_bytes = to_bytes(body, usize::MAX).await?;
    
    // Создаем тело запроса
    let body = http_body_util::Full::new(body_bytes)
        .map_err(|never| match never {})
        .boxed();

    // Выполняем запрос
    let response = client
        .request(new_req.body(body)?)
        .await?;

    // Конвертируем response и добавляем заголовок с целевым URL
    let (mut parts, body) = response.into_parts();
    parts.headers.insert("x-proxy-target", new_uri.parse()?);
    let body_bytes = body.collect().await?.to_bytes();
    Ok(Response::from_parts(parts, Body::from(body_bytes)))
} 