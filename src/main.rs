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
use crate::middleware::anti_kiddie::{anti_kiddie_middleware, cleanup_old_entries};
use crate::middleware::cache::{cache_middleware, warm_cache, cleanup_cache_stats};
use crate::services::beatmap_queue::processor::BeatmapProcessor;
use crate::services::osu_api::OsuApiService;
use crate::services::status::start_background_metrics_task;
use axum::{middleware::from_fn, Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;
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

    // Démarrer les tâches de nettoyage
    tokio::spawn(cleanup_old_entries());
    info!("🛡️ Anti-kiddie cleanup task started");
    
    tokio::spawn(cleanup_cache_stats());
    info!("💾 Cache stats cleanup task started");
    
    // Pré-chauffer le cache
    tokio::spawn(warm_cache());
    info!("🔥 Cache warming task started");

    let app = Router::new()
        .merge(routes::create_router(db))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn(cache_middleware))        // Cache en premier (plus proche de la réponse)
                .layer(from_fn(anti_kiddie_middleware))  // Sécurité après cache
                .layer(CorsLayer::permissive())
        );

    let app = setup_middleware(app);

    let addr: SocketAddr = config
        .server_address()
        .parse()
        .expect("Invalid server address");
    info!("listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
