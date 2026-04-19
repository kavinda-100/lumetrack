use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Shared state to hold our Redis client
struct AppState {
    redis_client: redis::Client,
}

#[derive(Debug, Deserialize, Serialize)]
struct LocationPing {
    driver_id: String,
    lat: f64,
    lng: f64,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    // Initialize tracing for high-visibility logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize Redis Client
    let redis_client = redis::Client::open("redis://127.0.0.1/").expect("Invalid Redis URL");
    let state = Arc::new(AppState { redis_client });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Telemetry service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    tracing::info!("New driver connected");

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // Client disconnected
            tracing::info!("Driver disconnected");
            return;
        };

        if let Message::Text(text) = msg {
            if let Ok(ping) = serde_json::from_str::<LocationPing>(text.as_str()) {
                let mut con = state
                    .redis_client
                    .get_multiplexed_async_connection()
                    .await
                    .unwrap();

                // 1. Store the raw metadata (for quick lookup)
                let key = format!("driver:{}:location", ping.driver_id);
                let _: () = con.set_ex(&key, text.as_str(), 60).await.unwrap();

                // 2. Add to Geospatial Index
                // Command: GEOADD key longitude latitude member
                let _: () = redis::cmd("GEOADD")
                    .arg("drivers:locations")
                    .arg(ping.lng)
                    .arg(ping.lat)
                    .arg(&ping.driver_id)
                    .query_async(&mut con)
                    .await
                    .unwrap();

                tracing::info!("Geospatial index updated for: {}", ping.driver_id);
            }
        }
    }
}
