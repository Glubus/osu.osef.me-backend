//! # Template Axum SQLx API
//!
//! Ce module est le point d'entrée principal de l'application.
//! Il configure et démarre le serveur HTTP avec Axum.
//!
//! ## Fonctionnalités
//! - Configuration depuis variables d'environnement (.env)
//! - Initialisation de la base de données
//! - Configuration du logging
//! - Configuration CORS
//! - Gestion des erreurs

mod config;
mod db;
mod handlers;
mod helpers;
mod middleware;
mod models;
mod routes;
mod services;

use crate::config::Config;
use crate::middleware::logging::setup_middleware;
use crate::services::beatmap_queue::processor::BeatmapProcessor;
use crate::services::osu_api::OsuApiService;
use crate::services::status::start_background_metrics_task;
use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    let config = Config::load().expect("Failed to load configuration");

    let mut db = db::DatabaseManager::new();
    db.connect(&config)
        .await
        .expect("Failed to connect to database");

    OsuApiService::initialize(
        config.osu_api.client_id,
        config.osu_api.client_secret.clone(),
    )
    .await
    .expect("Failed to initialize OsuApiService");
    start_background_metrics_task(db.clone(), config.clone()).await;
    info!("Background metrics task started (5-minute intervals)");

    BeatmapProcessor::initialize(db.clone());
    info!("BeatmapProcessor initialized");
    BeatmapProcessor::instance().start_processing_thread();
    info!("BeatmapProcessor thread started");

    let app = Router::new()
        .merge(routes::create_router(db))
        .layer(CorsLayer::permissive());

    let app = setup_middleware(app);

    let addr: SocketAddr = config
        .server_address()
        .parse()
        .expect("Invalid server address");
    info!("listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
