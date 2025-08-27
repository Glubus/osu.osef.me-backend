use axum::{extract::State, http::StatusCode, response::Json};
use chrono::Utc;
use std::time::Instant;

use crate::{
    db::DatabaseManager,
    helpers::help::{check_database_health, get_system_metrics},
    models::help::{HealthResponse, PerformanceMetrics},
};

#[utoipa::path(
    get,
    path = "/api/help/health",
    tag = "System",
    responses(
        (status = 200, description = "System is healthy", body = HealthResponse),
        (status = 503, description = "System is unhealthy")
    ),
    summary = "Get system health status",
    description = "Performs a comprehensive health check of the system including database connection, system metrics, and performance metrics."
)]
pub async fn health_check(
    State(db): State<DatabaseManager>,
) -> Result<Json<HealthResponse>, StatusCode> {
    let start_time = Instant::now();
    let db_status = check_database_health(&db).await;

    let system_metrics = get_system_metrics();

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
