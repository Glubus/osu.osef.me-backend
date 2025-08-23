use crate::db::DatabaseManager;
use crate::models::beatmap::beatmap::{Beatmap, BeatmapWithMSD, BeatmapWithMSDShort, Filters};
use crate::models::beatmap::extended::BeatmapsetCompleteExtended;
use crate::models::beatmap::pending_beatmap::PendingBeatmap;
use anyhow::Result;
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::Response,
};
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

#[derive(Serialize)]
pub struct BeatmapFiltersResponse {
    pub beatmaps: Vec<BeatmapWithMSDShort>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

pub async fn batch_checksums_handler(
    State(_db): State<DatabaseManager>,
    Json(payload): Json<BatchChecksumsRequest>,
) -> Result<Json<BatchChecksumsResponse>, StatusCode> {
    tracing::info!(
        "batch_checksums_handler: received {} checksums",
        payload.checksums.len()
    );
    for checksum in payload.checksums {
        let _ = PendingBeatmap::insert(_db.get_pool(), &checksum)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(BatchChecksumsResponse {
        message: "Batch ajouté à la queue de traitement".to_string(),
        status: "success".to_string(),
    }))
}

pub async fn beatmap_filters_handler(
    State(db): State<DatabaseManager>,
    Query(query): Query<Filters>,
) -> Result<Json<BeatmapFiltersResponse>, StatusCode> {
    let pool = db.get_pool();

    // Pagination
    let per_page = query.per_page.unwrap_or(30);
    let page = query.page.unwrap_or(1);

    // Récupérer le total d'abord
    let total = BeatmapWithMSDShort::count_by_filters(pool, &query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Récupérer les beatmaps filtrés
    let beatmaps = BeatmapWithMSDShort::find_by_filters(pool, &query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Calculer le nombre total de pages
    let total_pages = (total + per_page - 1) / per_page;

    Ok(Json(BeatmapFiltersResponse {
        beatmaps,
        total,
        page,
        per_page,
        total_pages,
    }))
}

pub async fn beatmap_by_id_handler(
    State(db): State<DatabaseManager>,
    Path(id): Path<i32>,
) -> Result<Json<BeatmapWithMSD>, StatusCode> {
    let pool = db.get_pool();

    // Récupérer le beatmap complet avec MSD et beatmapset
    let beatmap_with_msd = BeatmapWithMSD::find_by_beatmap_id(pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match beatmap_with_msd {
        Some(beatmap) => Ok(Json(beatmap)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn beatmap_osu_file_handler(
    State(db): State<DatabaseManager>,
    Path(id): Path<i32>,
) -> Result<Response<String>, StatusCode> {
    let pool = db.get_pool();

    // Récupérer le beatmap pour obtenir le file_path
    let beatmap = Beatmap::find_by_id(pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let beatmap = match beatmap {
        Some(b) => b,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Utiliser le file_path déjà stocké dans la base
    let osu_url = beatmap.file_path;

    // Récupérer le contenu du fichier
    let response = reqwest::get(&osu_url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content = response
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Retourner le contenu avec les bons headers
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}


pub async fn beatmapset_by_id_handler(
    State(db): State<DatabaseManager>,
    Path(id): Path<i32>,
) -> Result<Json<BeatmapsetCompleteExtended>, StatusCode> {
    let pool = db.get_pool();
    let beatmapset = BeatmapsetCompleteExtended::find_by_beatmapset_osu_id(pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match beatmapset {
        Some(beatmapset) => Ok(Json(beatmapset)),
        None => Err(StatusCode::NOT_FOUND),
    }
}