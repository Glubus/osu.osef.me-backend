use crate::db::DatabaseManager;
use crate::models::pending_beatmap::PendingBeatmap;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BatchChecksumsRequest {
    pub checksums: Vec<String>,
}

#[derive(Serialize)]
pub struct BatchChecksumsResponse {
    pub message: String,
    pub status: String,
}

pub async fn handler(
    State(db): State<DatabaseManager>,
    Json(payload): Json<BatchChecksumsRequest>,
) -> Result<Json<BatchChecksumsResponse>, StatusCode> {
    let batch: Vec<String> = payload.checksums.into_iter().take(50).collect();

    if batch.is_empty() {
        return Ok(Json(BatchChecksumsResponse {
            message: "No checksum provided".to_string(),
            status: "400".to_string(),
        }));
    }

    let inserted = PendingBeatmap::bulk_insert(db.get_pool(), &batch)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(BatchChecksumsResponse {
        message: format!("{} checksums added to processing queue", inserted),
        status: "200".to_string(),
    }))
}
