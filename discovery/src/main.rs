use std::sync::Arc;

mod config;
mod controllers;
mod routes;
use crate::{config::env::EnvConfig, routes::create_routes};

#[derive(Clone)]
struct AppState {
    redis_client: redis::Client,
    #[allow(dead_code)]
    env_config: EnvConfig,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let env = EnvConfig::from_env();

    let redis_client =
        redis::Client::open(env.redis_client_url.clone()).expect("Invalid Redis URL");

    let state = Arc::new(AppState {
        redis_client,
        env_config: env.clone(),
    });

    // let app = Router::new()
    //     .route("/api/v1/search-service/search", get(find_nearby_drivers))
    //     .with_state(state);

    let app = create_routes(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], env.port));
    tracing::info!("Discovery service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
