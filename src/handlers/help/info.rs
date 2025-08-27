use crate::models::help::{EndpointInfo, InfoResponse};
use axum::response::Json;

#[utoipa::path(
    get,
    path = "/api/help/info",
    tag = "System",
    responses(
        (status = 200, description = "API information retrieved successfully", body = InfoResponse)
    ),
    summary = "Get API information",
    description = "Retrieves general information about the API including version, description, and available endpoints."
)]
pub async fn info() -> Json<InfoResponse> {
    Json(InfoResponse {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        authors: env!("CARGO_PKG_AUTHORS")
            .split(':')
            .map(|s| s.trim().to_string())
            .collect(),
        endpoints: vec![
            EndpointInfo {
                path: "/help/health".to_string(),
                method: "GET".to_string(),
                description: "Vérification complète de l'état de santé du système".to_string(),
            },
            EndpointInfo {
                path: "/help/health-light".to_string(),
                method: "GET".to_string(),
                description: "Vérification rapide (DB + performance seulement)".to_string(),
            },
            EndpointInfo {
                path: "/help/info".to_string(),
                method: "GET".to_string(),
                description: "Informations sur l'API".to_string(),
            },
            EndpointInfo {
                path: "/help/ping".to_string(),
                method: "GET".to_string(),
                description: "Test de connectivité simple".to_string(),
            },
        ],
    })
}
