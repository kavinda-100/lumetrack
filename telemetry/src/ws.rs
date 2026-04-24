use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationPing {
    pub driver_id: String,
    pub lat: f64,
    pub lng: f64,
    pub timestamp: u64,
}

/// WebSocket handler for incoming driver location pings
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Core logic to handle each WebSocket connection and process incoming location pings
pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
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
                // Establish connection once per driver session
                let mut con = match state.redis_client.get_multiplexed_async_connection().await {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!("Failed to connect to Redis: {}", e);
                        return; // Close this driver's socket since can't save their data
                    }
                };

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
                    .unwrap_or_else(|e| {
                        tracing::error!("Failed to execute GEOADD: {}", e);
                    });

                tracing::info!("Geospatial index updated for: {}", ping.driver_id);
            }
        }
    }
}
