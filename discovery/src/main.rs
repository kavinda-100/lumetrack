use axum::{
    Json, Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
};
use redis::FromRedisValue;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

mod config;
use crate::config::env::EnvConfig;
struct AppState {
    redis_client: redis::Client,
    #[allow(dead_code)]
    env_config: EnvConfig,
}

#[derive(Deserialize)]
struct SearchParams {
    lat: f64,
    lng: f64,
    radius_km: f64,
}

#[derive(Serialize)]
struct DriverMatch {
    driver_id: String,
    distance_km: f64,
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

    let app = Router::new()
        .route("/api/v1/search-service/search", get(find_nearby_drivers))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], env.port));
    tracing::info!("Discovery service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn find_nearby_drivers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    // Establish connection once per driver session
    let mut con = match state.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to connect to Redis: {}", e);
            return Json(Vec::<DriverMatch>::new());
        }
    };

    // Use GEOSEARCH to find members in the "drivers:locations" key
    let results: Vec<Vec<redis::Value>> = redis::cmd("GEOSEARCH")
        .arg("drivers:locations")
        .arg("FROMLONLAT")
        .arg(params.lng)
        .arg(params.lat)
        .arg("BYRADIUS")
        .arg(params.radius_km)
        .arg("KM")
        .arg("WITHDIST")
        .arg("ASC")
        .query_async(&mut con)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to execute GEOSEARCH: {}", e);
            Vec::new()
        });

    let matches: Vec<DriverMatch> = results
        .into_iter()
        .filter_map(|res| {
            // Destructure the vector into an array if it matches the pattern
            let [id_val, dist_val] = <[redis::Value; 2]>::try_from(res).ok()?;

            let id = String::from_redis_value(id_val).ok()?;
            let dist = f64::from_redis_value(dist_val).ok()?;

            Some(DriverMatch {
                driver_id: id,
                distance_km: dist,
            })
        })
        .collect();

    Json(matches)
}
