use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::{StatusCode, uri::Uri},
    response::IntoResponse,
    routing::any,
};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use std::net::SocketAddr;
use std::sync::Arc;

// We use an Arc to share the HTTP client across all threads efficiently
type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

struct AppState {
    client: Client,
}

#[tokio::main]
async fn main() {
    // 1. Initialize Logging
    tracing_subscriber::fmt::init();

    // 2. Setup the high-performance Hyper client
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new())
        .build(HttpConnector::new());

    let state = Arc::new(AppState { client });

    // 3. Define the Router
    let app = Router::new()
        // Capture everything under /api/v1 and send to our handler
        .route("/api/v1/{*path}", any(proxy_handler))
        .with_state(state);

    // 4. Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::info!("LumeTrack Gateway running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn proxy_handler(State(state): State<Arc<AppState>>, mut req: Request) -> impl IntoResponse {
    let path = req.uri().path();

    // Determine which microservice to target based on the path
    let target_url = if path.starts_with("/api/v1/orders") {
        "http://127.0.0.1:5003" // TS Order Manager
    } else if path.starts_with("/api/v1/telemetry") {
        "http://127.0.0.1:5001" // Rust Telemetry
    } else {
        return StatusCode::NOT_FOUND.into_response();
    };

    // Construct the new URI for the internal service
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let new_uri = format!("{}{}", target_url, path_query);

    // Rewrite the request URI
    *req.uri_mut() = Uri::try_from(new_uri).unwrap();

    // Forward the request using the Hyper client
    match state.client.request(req).await {
        Ok(res) => res.into_response(),
        Err(err) => {
            tracing::error!("Proxy error: {}", err);
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}
