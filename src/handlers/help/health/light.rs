use axum::{extract::State, http::StatusCode, response::Json};
use chrono::Utc;
use std::time::Instant;
use sysinfo::System;

use crate::{
    db::DatabaseManager,
    helpers::help::check_database_health,
    models::help::{HealthResponse, PerformanceMetrics, SystemMetrics},
};

#[utoipa::path(
    get,
    path = "/api/help/health-light",
    tag = "System",
    responses(
        (status = 200, description = "System is healthy", body = HealthResponse),
        (status = 503, description = "System is unhealthy")
    ),
    summary = "Get light system health status",
    description = "Performs a quick health check focusing only on database connection and basic performance metrics."
)]
pub async fn health_light(
    State(db): State<DatabaseManager>,
) -> Result<Json<HealthResponse>, StatusCode> {
    let start_time = Instant::now();

    // Vérification de la base de données seulement
    let db_status = check_database_health(&db).await;

    // Métriques système minimales
    let system_metrics = SystemMetrics {
        cpu_usage: 0.0, // Skip CPU check for speed
        cpu_count: 0,
        memory_used_mb: 0,
        memory_total_mb: 0,
        memory_usage_percent: 0.0,
        disk_usage_percent: 0.0,
        uptime: System::uptime(),
    };

    // Métriques de performance
    let response_time = start_time.elapsed().as_millis() as u64;
    let performance_metrics = PerformanceMetrics {
        response_time_ms: response_time,
    };

    let health_response = HealthResponse {
        status: if db_status.connected {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
        system: system_metrics,
        performance: performance_metrics,
    };

    if health_response.status == "healthy" {
        Ok(Json(health_response))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
