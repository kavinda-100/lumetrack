use axum::{
    Router,
    body::Body,
    extract::{Request, State, ws::WebSocketUpgrade},
    http::{StatusCode, uri::Uri},
    response::IntoResponse,
    routing::{any, get},
};
use futures_util::{SinkExt, StreamExt};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_tungstenite::connect_async;

mod config;
use crate::config::env::EnvConfig; // Import directly from rt

// Type alias for the HTTP client used for proxying requests
type Client =
    hyper_util::client::legacy::Client<hyper_util::client::legacy::connect::HttpConnector, Body>;

/// Shared application state containing the HTTP client and environment configuration
#[derive(Clone)]
struct AppState {
    client: Client,
    env: EnvConfig,
}

#[tokio::main]
async fn main() {
    // Load .env file if present (for local development)
    dotenvy::dotenv().ok();

    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create a shared HTTP client for proxying requests
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new())
        .build(HttpConnector::new());

    // Load configuration from environment variables
    let env_config = EnvConfig::from_env();

    // Create shared application state
    let state = Arc::new(AppState {
        client,
        env: env_config.clone(),
    });

    // Build the Axum application with routes
    let app = Router::new()
        // Special route for WebSockets (Telemetry)
        .route("/ws/v1/telemetry-service", get(ws_proxy_handler))
        // General API routes
        .route("/api/v1/{*path}", any(proxy_handler))
        .with_state(state);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], env_config.port.clone()));
    tracing::info!("LumeTrack Gateway active on port {}", env_config.port);

    // run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handles standard REST calls
async fn proxy_handler(State(state): State<Arc<AppState>>, mut req: Request) -> impl IntoResponse {
    // Determine target service based on request path
    let path = req.uri().path();

    // find the target port based on the path prefix
    let target_port = if path.starts_with("/api/v1/orders-service") {
        state.env.order_service_port // Order Service
    } else if path.starts_with("/api/v1/search-service") {
        state.env.discovery_service_port // Discovery Service
    } else if path.starts_with("/api/v1/identity-service") {
        state.env.identity_service_port // Identity Service
    } else {
        return StatusCode::NOT_FOUND.into_response();
    };

    // Reconstruct the full target URL by combining the main URL, target port, and original path/query
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    // e.g. http://main_url:5002/api/v1/some-endpoint?query
    let new_uri = format!("{}{}{}", state.env.main_url, target_port, path_query);

    *req.uri_mut() = Uri::try_from(new_uri).unwrap();

    match state.client.request(req).await {
        Ok(res) => res.into_response(),
        Err(_) => StatusCode::BAD_GATEWAY.into_response(),
    }
}

// Handles WebSocket Tunnelling for Telemetry Service
async fn ws_proxy_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        // Connect to the internal Telemetry service
        let target_ws_url = state.env.telemetry_service_ws_url.clone();

        let (backend_ws, _) = match connect_async(target_ws_url).await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to connect to backend WS: {}", e);
                return;
            }
        };

        let (mut backend_ws_sink, mut backend_ws_stream) = backend_ws.split();
        let (mut client_ws_sink, mut client_ws_stream) = socket.split();

        // Bidirectional Tunnel: Pipe client -> backend and backend -> client
        let f1 = async {
            while let Some(Ok(msg)) = client_ws_stream.next().await {
                // Convert Axum message to Tungstenite message
                let m = tokio_tungstenite::tungstenite::Message::from(msg.into_data());
                if backend_ws_sink.send(m).await.is_err() {
                    break;
                }
            }
        };

        let f2 = async {
            while let Some(Ok(msg)) = backend_ws_stream.next().await {
                // Convert Tungstenite message to Axum message
                let data = msg.into_data();
                if client_ws_sink
                    .send(axum::extract::ws::Message::Binary(data))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        };

        tokio::join!(f1, f2);
    })
}
