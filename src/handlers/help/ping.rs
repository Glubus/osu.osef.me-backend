#[utoipa::path(
    get,
    path = "/api/help/ping",
    tag = "System",
    responses(
        (status = 200, description = "API is reachable", body = String)
    ),
    summary = "Ping the API",
    description = "Simple endpoint to check if the API is reachable. Returns 'pong' if successful."
)]
pub async fn ping() -> &'static str {
    "pong"
}
