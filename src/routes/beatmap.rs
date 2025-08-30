//! # Beatmap Routes Module
//!
//! Ce module configure les routes de beatmap.

use crate::{db::DatabaseManager, handlers};
use axum::{
    Router,
    routing::{get, post},
};

pub fn router(db: DatabaseManager) -> Router<DatabaseManager> {
    Router::new()
        .route(
            "/beatmap/batch",
            post(handlers::beatmap::batch::checksums::handler),
        )
        .route(
            "/beatmap/by_osu_id",
            post(handlers::beatmap::post::by_beatmap_id::handler),
        )
        .route(
            "/beatmap",
            get(handlers::beatmap::get::filtered::handler),
        )
        .route(
            "/beatmap/random",
            get(handlers::beatmap::get::random::handler),
        )
        .route(
            "/beatmapset/{id}",
            get(handlers::beatmap::get::by_id_extended::handler),
        )
        .route(
            "/beatmap/count",
            get(handlers::beatmap::get::count::handler),
        )
        .with_state(db)
}
