use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use redis::FromRedisValue;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    lat: f64,
    lng: f64,
    radius_km: f64,
}

#[derive(Serialize)]
pub struct DriverMatch {
    driver_id: String,
    distance_km: f64,
}

pub async fn find_nearby_drivers(
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
