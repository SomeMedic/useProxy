mod config;
mod proxy;
mod logger;
mod cert;
mod cli;

use clap::Parser;
use config::Config;
use proxy::ProxyServer;
use logger::Logger;
use tokio;
use tracing::info;
use tracing_subscriber::fmt;
use std::sync::Arc;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логирования
    fmt::init();

    // Парсим аргументы командной строки
    let cli = cli::Cli::parse();

    // Загружаем конфигурацию
    let mut config = if let Some(config_path) = cli.config {
        Config::load_from_file(&config_path)?
    } else {
        Config::load()?
    };

    // Обрабатываем команды
    match cli.command {
        Some(cli::Commands::Run { port, host, https }) => {
            // Обновляем конфигурацию из аргументов командной строки
            if let Some(port) = port {
                config.port = port;
            }
            if let Some(host) = host {
                config.host = host;
            }
            config.https.enabled = https;

            start_server(config).await?;
        }
        Some(cli::Commands::Cert { command }) => {
            match command {
                cli::CertCommands::Gen { cert, key } => {
                    let cert_path = cert.unwrap_or_else(|| Path::new("cert.pem").to_path_buf());
                    let key_path = key.unwrap_or_else(|| Path::new("key.pem").to_path_buf());
                    cert::generate_self_signed_cert(&cert_path, &key_path)?;
                    println!("Сертификаты успешно сгенерированы:");
                    println!("Сертификат: {}", cert_path.display());
                    println!("Приватный ключ: {}", key_path.display());
                }
            }
        }
        Some(cli::Commands::Proxy { command }) => {
            match command {
                cli::ProxyCommands::Add { rule, change_origin } => {
                    let parts: Vec<&str> = rule.split("->").collect();
                    if parts.len() != 2 {
                        return Err("Неверный формат правила. Используйте: путь -> целевой_url".into());
                    }
                    let path = parts[0].trim().to_string();
                    let target = parts[1].trim().to_string();
                    
                    config.proxy.rules.push(config::ProxyRule {
                        path,
                        target,
                        change_origin,
                    });
                    config.save()?;
                    println!("Правило прокси добавлено");
                }
                cli::ProxyCommands::Remove { path } => {
                    config.proxy.rules.retain(|rule| rule.path != path);
                    config.save()?;
                    println!("Правило прокси удалено");
                }
                cli::ProxyCommands::List => {
                    println!("Правила прокси:");
                    for rule in &config.proxy.rules {
                        println!("{} -> {} (change_origin: {})", rule.path, rule.target, rule.change_origin);
                    }
                }
                cli::ProxyCommands::Clear => {
                    config.proxy.rules.clear();
                    config.save()?;
                    println!("Все правила прокси удалены");
                }
            }
        }
        Some(cli::Commands::Logs { command }) => {
            match command {
                cli::LogCommands::Show => {
                    println!("Для просмотра логов откройте: http{}://localhost:{}/logs",
                        if config.https.enabled { "s" } else { "" },
                        config.port
                    );
                }
                cli::LogCommands::Clear => {
                    let logger = Arc::new(Logger::new());
                    logger.clear_logs().await;
                    println!("Логи очищены");
                }
                cli::LogCommands::Filter { method, status, path } => {
                    let logger = Arc::new(Logger::new());
                    let logs = logger.filter_logs(method, status, path).await;
                    println!("Отфильтрованные логи:");
                    for log in logs {
                        println!("{} {} {} -> {}", log.timestamp, log.method, log.path, log.status);
                    }
                }
            }
        }
        None => {
            // Запускаем сервер с настройками по умолчанию
            start_server(config).await?;
        }
    }

    Ok(())
}

async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting UseProxy server...");
    info!(
        "Loaded configuration: static_files={}, proxy_rules={}, https={}",
        config.static_files.enabled,
        config.proxy.rules.len(),
        config.https.enabled
    );

    // Если HTTPS включен, проверяем наличие сертификатов
    if config.https.enabled {
        let cert_path = Path::new(&config.https.cert_file);
        let key_path = Path::new(&config.https.key_file);

        if !cert_path.exists() || !key_path.exists() {
            info!("Generating self-signed certificates...");
            cert::generate_self_signed_cert(cert_path, key_path)?;
            info!("Certificates generated successfully");
        }
    }

    // Создание логгера
    let logger = Arc::new(Logger::new());
    info!("Logger initialized");

    // Создание прокси-сервера
    let proxy_server = ProxyServer::new(config.clone(), logger.clone());
    let app = proxy_server.router();

    // Запуск сервера
    let addr = config.server_addr();
    
    match config.load_tls_config().await? {
        Some(tls_config) => {
            info!("Starting HTTPS server on https://{}", addr);
            axum_server::bind_rustls(addr.parse()?, tls_config)
                .serve(app.into_make_service())
                .await?;
        }
        None => {
            info!("Starting HTTP server on http://{}", addr);
            axum_server::bind(addr.parse()?)
                .serve(app.into_make_service())
                .await?;
        }
    }

    Ok(())
} 