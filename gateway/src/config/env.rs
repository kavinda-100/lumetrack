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
    pub port: u16,                        // 5000 by default
    pub order_service_port: u16,          // 5003 by default
    pub discovery_service_port: u16,      // 5002 by default
    pub identity_service_port: u16,       // 5004 by default
    pub telemetry_service_ws_url: String, // e.g. ws://localhost:5000/ws/v1/telemetry-service
    pub main_url: String,                 // e.g. http://127.0.0.1:
}

impl EnvConfig {
    pub fn from_env() -> Self {
        let dev_mode_raw = env::var("DEV_MODE").unwrap_or_else(|_| "development".to_string());
        let dev_mode = DevMode::from_env_value(&dev_mode_raw);
        let port = env::var("PORT")
            .unwrap_or_else(|_| "5000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");
        let order_service_port = env::var("ORDER_SERVICE_PORT")
            .unwrap_or_else(|_| "5003".to_string())
            .parse::<u16>()
            .expect("ORDER_SERVICE_PORT must be a valid number");
        let discovery_service_port = env::var("DISCOVERY_SERVICE_PORT")
            .unwrap_or_else(|_| "5002".to_string())
            .parse::<u16>()
            .expect("DISCOVERY_SERVICE_PORT must be a valid number");
        let identity_service_port = env::var("IDENTITY_SERVICE_PORT")
            .unwrap_or_else(|_| "5004".to_string())
            .parse::<u16>()
            .expect("IDENTITY_SERVICE_PORT must be a valid number");
        let telemetry_service_ws_url = env::var("TELEMETRY_SERVICE_WS_URL")
            .unwrap_or_else(|_| "ws://127.0.0.1:5001/ws".to_string());
        let main_url = env::var("MAIN_URL").unwrap_or_else(|_| "http://127.0.0.1:".to_string());

        EnvConfig {
            dev_mode,
            port,
            order_service_port,
            discovery_service_port,
            identity_service_port,
            telemetry_service_ws_url,
            main_url,
        }
    }
}
