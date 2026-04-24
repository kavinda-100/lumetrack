use axum::{Json, Router, response::IntoResponse, routing::get};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod ws; // WebSocket handling logic
use crate::config::env::EnvConfig;
use crate::ws::ws_handler; // Import directly from rt

// Shared state to hold our Redis client
#[derive(Clone)]
struct AppState {
    redis_client: redis::Client,
    #[allow(dead_code)]
    env_config: EnvConfig,
}

#[tokio::main]
async fn main() {
    // Initialize tracing for high-visibility logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment variables
    let env_config = EnvConfig::from_env();

    // Initialize Redis Client
    let redis_client =
        redis::Client::open(env_config.redis_client_url.clone()).expect("Invalid Redis URL");

    // Create shared application state
    let state = Arc::new(AppState {
        redis_client,
        env_config: env_config.clone(),
    });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/health", get(health_check_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], env_config.port));
    tracing::info!("Telemetry service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// health check endpoint for monitoring
async fn health_check_handler() -> impl IntoResponse {
    // send a sJson response with status "ok", and the current timestamp
    let response = serde_json::json!({
        "status": "ok",
        "message": "Gateway is healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    Json(response).into_response()
}
