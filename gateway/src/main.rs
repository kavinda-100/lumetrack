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
use tokio_tungstenite::connect_async; // Import directly from rt

type Client =
    hyper_util::client::legacy::Client<hyper_util::client::legacy::connect::HttpConnector, Body>;

struct AppState {
    client: Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new())
        .build(HttpConnector::new());

    let state = Arc::new(AppState { client });

    let app = Router::new()
        // Special route for WebSockets (Telemetry)
        .route("/ws/v1/telemetry-service", get(ws_proxy_handler))
        // General API routes
        .route("/api/v1/{*path}", any(proxy_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    tracing::info!("LumeTrack Gateway active on port 5000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handles standard REST calls
async fn proxy_handler(State(state): State<Arc<AppState>>, mut req: Request) -> impl IntoResponse {
    let path = req.uri().path();

    let target_port = if path.starts_with("/api/v1/orders-service") {
        5003 // Order Manager
    } else if path.starts_with("/api/v1/search-service") {
        5002 // Discovery Service
    } else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let new_uri = format!("http://127.0.0.1:{}{}", target_port, path_query);

    *req.uri_mut() = Uri::try_from(new_uri).unwrap();

    match state.client.request(req).await {
        Ok(res) => res.into_response(),
        Err(_) => StatusCode::BAD_GATEWAY.into_response(),
    }
}

// Handles WebSocket Tunnelling for Telemetry Service
async fn ws_proxy_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        // Connect to the internal Telemetry service
        let target_ws_url = "ws://127.0.0.1:5001/ws";

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
