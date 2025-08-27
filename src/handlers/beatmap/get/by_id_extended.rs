use axum::{extract::State, Json, http::StatusCode, extract::Path};
use serde::{Serialize};
use crate::models::extended::complete::types::BeatmapsetCompleteExtended;
use crate::{db::DatabaseManager};

#[derive(Serialize)]
pub struct BeatmapByIdExtendedResponse {
    pub beatmap: BeatmapsetCompleteExtended,
}

pub async fn handler(
    State(db): State<DatabaseManager>,
    Path(id): Path<i32>,
) -> Result<Json<BeatmapByIdExtendedResponse>, StatusCode> {
    let pool = db.get_pool();

    let beatmap = BeatmapsetCompleteExtended::find_by_beatmapset_osu_id(pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;


    Ok(Json(BeatmapByIdExtendedResponse {
        beatmap: beatmap.unwrap(),
    }))
}
