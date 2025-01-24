use tower_http::cors::{CorsLayer, Any};
use crate::config::CorsConfig;

pub fn create_cors_layer(config: &CorsConfig) -> CorsLayer {
    let mut layer = CorsLayer::new();

    // Настраиваем разрешенные origins
    if config.allowed_origins.contains(&"*".to_string()) {
        layer = layer
            .allow_origin(Any)
            .allow_credentials(false); // Отключаем credentials при использовании "*"
    } else {
        layer = layer
            .allow_origin(config.allowed_origins.iter().map(|o| o.parse().unwrap()).collect::<Vec<_>>())
            .allow_credentials(true);
    }

    // Настраиваем разрешенные методы
    layer = layer.allow_methods(
        config.allowed_methods.iter()
            .map(|m| m.parse().unwrap())
            .collect::<Vec<_>>()
    );

    // Настраиваем разрешенные заголовки
    if config.allowed_headers.contains(&"*".to_string()) {
        layer = layer.allow_headers(Any);
    } else {
        layer = layer.allow_headers(
            config.allowed_headers.iter()
                .map(|h| h.parse().unwrap())
                .collect::<Vec<_>>()
        );
    }

    // Включаем expose headers
    layer = layer.expose_headers(Any);

    layer
} 