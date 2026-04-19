use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use redis::{AsyncCommands, FromRedisValue};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

struct AppState {
    redis_client: redis::Client,
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
    tracing_subscriber::fmt::init();

    let redis_client = redis::Client::open("redis://127.0.0.1/").expect("Invalid Redis URL");
    let state = Arc::new(AppState { redis_client });

    let app = Router::new()
        .route("/search", get(find_nearby_drivers))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8081));
    tracing::info!("Discovery service listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn find_nearby_drivers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    let mut con = state.redis_client.get_multiplexed_async_connection().await.unwrap();

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
        .unwrap();

    let matches: Vec<DriverMatch> = results
        .into_iter()
        .filter_map(|res| {
            if res.len() >= 2 {
                let id = String::from_redis_value(&res[0]).ok()?;
                let dist = f64::from_redis_value(&res[1]).ok()?;
                Some(DriverMatch { driver_id: id, distance_km: dist })
            } else {
                None
            }
        })
        .collect();

    Json(matches)
}