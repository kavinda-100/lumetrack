use axum::{Json, response::IntoResponse};

pub mod search_controller;

// health check endpoint for monitoring
pub async fn health_check_handler() -> impl IntoResponse {
    // send a sJson response with status "ok", and the current timestamp
    let response = serde_json::json!({
        "status": "ok",
        "message": "Discovery service is healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    Json(response).into_response()
}
