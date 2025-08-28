use axum::{extract::State, Json, http::StatusCode, extract::Query};
use serde::{Serialize};
use crate::{db::DatabaseManager};
use crate::models::short::complete::types::BeatmapsetCompleteShort;
use crate::models::Filters;

#[derive(Serialize)]
pub struct BeatmapRandomResponse {
    pub beatmaps: Vec<BeatmapsetCompleteShort>,
    pub count: usize,
}

pub async fn handler(
    State(db): State<DatabaseManager>,
    Query(query): Query<Filters>,
) -> Result<Json<BeatmapRandomResponse>, StatusCode> {
    let pool = db.get_pool();

    let beatmaps = BeatmapsetCompleteShort::random_by_filters(pool, &query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let len = beatmaps.len();
    Ok(Json(BeatmapRandomResponse {
        beatmaps,
        count: len,
    }))
}
