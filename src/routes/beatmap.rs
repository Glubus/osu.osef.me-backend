use crate::db::DatabaseManager;
use crate::handlers::beatmap::{
    batch_checksums_handler, beatmap_by_id_handler, beatmap_filters_handler,
    beatmap_osu_file_handler,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/beatmap/batch", post(batch_checksums_handler))
        .route("/beatmap/filters", get(beatmap_filters_handler))
        .route("/beatmap/{id}", get(beatmap_by_id_handler))
        .route("/beatmap/{id}/osu", get(beatmap_osu_file_handler))
}
