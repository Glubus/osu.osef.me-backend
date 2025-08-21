//! # Template Axum SQLx API
//!
//! Ce module est le point d'entrée principal de l'application.
//! Il configure et démarre le serveur HTTP avec Axum.
//!
//! ## Fonctionnalités
//! - Configuration depuis les variables d'environnement (.env)
//! - Initialisation de la base de données
//! - Configuration du logging
//! - Configuration CORS
//! - Gestion des erreurs

use api::middleware::logging::setup_middleware;
use api::models::status::start_background_metrics_task;
use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

/// Point d'entrée principal de l'application.
///
/// Cette fonction :
/// 1. Charge la configuration depuis les variables d'environnement
/// 2. Initialise la base de données
/// 3. Configure les routes et les middlewares
/// 4. Démarre le serveur HTTP
#[tokio::main]
async fn main() {
    // Load configuration from environment variables
    let config = api::config::Config::load().expect("Failed to load configuration");

    // Initialize database
    let mut db = api::db::DatabaseManager::new();
    db.connect(&config)
        .await
        .expect("Failed to connect to database");

    // Initialize global OsuApiService
    api::services::osu_api::OsuApiService::initialize(
        config.osu_api.client_id,
        config.osu_api.client_secret.clone(),
    )
    .await
    .expect("Failed to initialize OsuApiService");

    // Initialize global BeatmapProcessor
    api::services::beatmap_processor::BeatmapProcessor::initialize(db.clone());

    // Démarrer la tâche de calcul des métriques en arrière-plan
    start_background_metrics_task(db.clone(), config.clone()).await;
    info!("Background metrics task started (5-minute intervals)");

    // Build our application with a route
    let app = Router::new()
        .merge(api::routes::create_router(db))
        .layer(CorsLayer::permissive());

    let app = setup_middleware(app);

    // Run it
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
