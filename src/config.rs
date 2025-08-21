use serde::{Deserialize, Serialize};
use std::env;
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsuApiConfig {
    pub client_id: u64,
    pub client_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
    pub osu_api: OsuApiConfig,
}

impl Config {
    /// Initialise le système de logging
    fn init_logging(level: &str, _format: &str) {
        let env_filter = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(level))
            .unwrap_or_else(|_| EnvFilter::new("info"));

        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();

        info!("Logging initialized with level: {}", level);
    }

    /// Charge la configuration depuis les variables d'environnement
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Charger le fichier .env s'il existe
        dotenv::dotenv().ok();

        // Charger les variables d'environnement
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .unwrap_or(3000);

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/template_db".to_string()
        });
        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .unwrap_or(10);
        let min_connections = env::var("DATABASE_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<u32>()
            .unwrap_or(1);

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let log_format = env::var("LOG_FORMAT").unwrap_or_else(|_| "json".to_string());

        // Parse CORS origins (comma-separated)
        let cors_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://127.0.0.1:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let cors_methods = env::var("CORS_ALLOWED_METHODS")
            .unwrap_or_else(|_| "GET,POST,PUT,DELETE,OPTIONS".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let cors_headers = env::var("CORS_ALLOWED_HEADERS")
            .unwrap_or_else(|_| "content-type,authorization".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let osu_client_id = env::var("OSU_CLIENT_ID")
            .unwrap_or_else(|_| "12345".to_string())
            .parse::<u64>()
            .unwrap_or(12345);
        let osu_client_secret =
            env::var("OSU_CLIENT_SECRET").unwrap_or_else(|_| "your_client_secret".to_string());

        let config = Config {
            server: ServerConfig {
                host: server_host,
                port: server_port,
            },
            database: DatabaseConfig {
                url: database_url,
                max_connections,
                min_connections,
            },
            logging: LoggingConfig {
                level: log_level,
                format: log_format,
            },
            cors: CorsConfig {
                allowed_origins: cors_origins,
                allowed_methods: cors_methods,
                allowed_headers: cors_headers,
            },
            osu_api: OsuApiConfig {
                client_id: osu_client_id,
                client_secret: osu_client_secret,
            },
        };

        // Initialiser le logging avec la configuration
        Self::init_logging(&config.logging.level, &config.logging.format);

        info!(
            "Configuration loaded successfully from environment variables. Server will bind to: {}",
            config.server_address()
        );
        Ok(config)
    }

    /// Retourne l'adresse complète du serveur
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        warn!("Using default configuration as no environment variables were found");
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "postgres://postgres:postgres@localhost:5432/template_db".to_string(),
                max_connections: 10,
                min_connections: 1,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
            cors: CorsConfig {
                allowed_origins: vec![
                    "http://localhost:3000".to_string(),
                    "http://127.0.0.1:3000".to_string(),
                ],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                allowed_headers: vec!["content-type".to_string(), "authorization".to_string()],
            },
            osu_api: OsuApiConfig {
                client_id: 12345,
                client_secret: "your_client_secret".to_string(),
            },
        }
    }
}
