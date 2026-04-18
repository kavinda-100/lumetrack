use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize, Serialize)]
struct LocationPing {
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

    let app = Router::new()
        .route("/ws", get(ws_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Telemetry service listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    tracing::info!("New driver connected");
    let mut counter = 0;

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // Client disconnected
            tracing::info!("Driver disconnected");
            return;
        };

        if let Message::Text(text) = msg {
            match serde_json::from_str::<LocationPing>(&text) {
                Ok(ping) => {
                    // Logic for Redis persistence or state updates goes here
                    tracing::info!("Location received: Lat {}, Lng {}", ping.lat, ping.lng);
                    counter += 1;
                    tracing::info!("Total locations received: {}", counter);
                }
                Err(e) => {
                    tracing::error!("Failed to parse telemetry data: {}", e);
                }
            }
        }
    }
}