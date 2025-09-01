//! # Help Routes Module
//!
//! Ce module configure les routes d'aide et de diagnostic de l'API.

use crate::{
    db::DatabaseManager,
    handlers::help::{
        health::{health_check, health_light},
        info, ping, security_stats, cache_stats,
    },
};
use axum::{Router, routing::get};

/// Créer le routeur pour les routes d'aide
pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/help/health", get(health_check))
        .route("/help/health-light", get(health_light))
        .route("/help/info", get(info))
        .route("/help/ping", get(ping))
        .route("/help/security-stats", get(security_stats))
        .route("/help/cache-stats", get(cache_stats))
}
