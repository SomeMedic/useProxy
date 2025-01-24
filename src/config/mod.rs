use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpsConfig {
    pub enabled: bool,
    pub cert_file: String,
    pub key_file: String,
}

impl Default for HttpsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_file: "cert.pem".to_string(),
            key_file: "key.pem".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StaticFilesConfig {
    pub enabled: bool,
    pub directory: String,
}

impl Default for StaticFilesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: "./public".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxyRule {
    pub path: String,
    pub target: String,
    pub change_origin: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxyConfig {
    pub rules: Vec<ProxyRule>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            rules: vec![ProxyRule {
                path: "/api/github".to_string(),
                target: "https://api.github.com".to_string(),
                change_origin: true,
            }],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub https: HttpsConfig,
    pub static_files: StaticFilesConfig,
    pub proxy: ProxyConfig,
    pub cors: CorsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "127.0.0.1".to_string(),
            https: HttpsConfig::default(),
            static_files: StaticFilesConfig::default(),
            proxy: ProxyConfig::default(),
            cors: CorsConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from_file(Path::new("config.json"))
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if path.exists() {
            let config_str = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&config_str)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to_file(Path::new("config.json"))
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config_str)?;
        Ok(())
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn load_tls_config(&self) -> Result<Option<axum_server::tls_rustls::RustlsConfig>, Box<dyn std::error::Error>> {
        if !self.https.enabled {
            return Ok(None);
        }

        let cert = tokio::fs::read(&self.https.cert_file).await?;
        let key = tokio::fs::read(&self.https.key_file).await?;

        let config = axum_server::tls_rustls::RustlsConfig::from_pem(cert, key).await?;
        Ok(Some(config))
    }
} 