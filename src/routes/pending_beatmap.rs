//! # Beatmap Routes Module
//!
//! Ce module configure les routes de beatmap.

use crate::{db::DatabaseManager, handlers};
use axum::{
    Router,
    routing::{get},
};


pub fn router(db: DatabaseManager) -> Router<DatabaseManager> {
    Router::new()
        .route(
            "/pending_beatmap/status/{id}",
            get(handlers::pending_beatmap::get::status_by_osu_id::handler),
        )
        .with_state(db)
}
