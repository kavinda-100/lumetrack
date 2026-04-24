use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevMode {
    Development,
    Test,
    Production,
}

impl DevMode {
    fn from_env_value(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "development" | "dev" => Self::Development,
            "test" => Self::Test,
            "production" | "prod" => Self::Production,
            _ => panic!("DEV_MODE must be one of: development|dev, test, production|prod"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvConfig {
    pub dev_mode: DevMode,
    pub port: u16,                // 5002 by default
    pub redis_client_url: String, // e.g. redis://127.0.0.1/
}

impl EnvConfig {
    pub fn from_env() -> Self {
        let dev_mode_raw = env::var("DEV_MODE").unwrap_or_else(|_| "development".to_string());
        let dev_mode = DevMode::from_env_value(&dev_mode_raw);
        let port = env::var("PORT")
            .unwrap_or_else(|_| "5002".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");
        let redis_client_url =
            env::var("REDIS_CLIENT_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

        EnvConfig {
            dev_mode,
            port,
            redis_client_url,
        }
    }
}
