use axum::{extract::State, http::StatusCode, response::Html};

use crate::{
    db::DatabaseManager,
    helpers::status::{
        display::{get_health_display, get_score_colors, get_status_info_from_metrics},
        fallback::generate_fallback_page,
        history::{generate_history_bars, generate_network_history_bars},
        network::get_network_metrics,
        utils::{format_uptime, get_load_average},
    },
    services::status::{get_history, get_metrics_with_fallback},
};

pub async fn status_page(State(_db): State<DatabaseManager>) -> Result<Html<String>, StatusCode> {
    let template = include_str!("../../../assets/status.html");

    let metrics = match get_metrics_with_fallback() {
        Some(m) => m,
        None => return Ok(Html(generate_fallback_page(template))),
    };

    let (health_color, health_icon, health_status) = get_health_display(metrics.health_score);
    let (score_color_start, score_color_end) = get_score_colors(metrics.health_score);
    let status_info = get_status_info_from_metrics(&metrics);

    let history = get_history();
    let history_bars = generate_history_bars(&history, "api");
    let db_history_bars = generate_history_bars(&history, "database");
    let network_history_bars = generate_network_history_bars(&history);

    let uptime_hours = metrics.uptime / 3600;
    let timestamp = metrics.timestamp.format("%H:%M").to_string();
    let (network_status, _, _) = get_network_metrics();

    let rendered = template
        .replace("{API_NAME}", env!("CARGO_PKG_NAME"))
        .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
        .replace("{TIMESTAMP}", &timestamp)
        .replace("{HEALTH_SCORE}", &metrics.health_score.to_string())
        .replace("{HEALTH_COLOR}", &health_color)
        .replace("{HEALTH_ICON}", &health_icon)
        .replace("{HEALTH_STATUS}", &health_status)
        .replace("{SCORE_COLOR_START}", &score_color_start)
        .replace("{SCORE_COLOR_END}", &score_color_end)
        .replace("{CPU_SCORE}", &metrics.cpu_score.to_string())
        .replace("{MEMORY_SCORE}", &metrics.memory_score.to_string())
        .replace("{PERF_SCORE}", &metrics.perf_score.to_string())
        .replace("{NETWORK_SCORE}", &metrics.network_score.to_string())
        .replace("{STATUS_BADGE}", &status_info.0)
        .replace("{STATUS_TEXT}", &status_info.1)
        .replace("{RESPONSE_TIME}", &metrics.response_time_ms.to_string())
        .replace("{UPTIME_HOURS}", &uptime_hours.to_string())
        .replace("{NETWORK_STATUS}", &network_status)
        .replace("{HISTORY_BARS_HTML}", &history_bars)
        .replace("{DB_HISTORY_BARS_HTML}", &db_history_bars)
        .replace("{NETWORK_HISTORY_BARS_HTML}", &network_history_bars)
        .replace("{THEME}", "retro")
        .replace("{UPTIME_FULL}", &format_uptime(metrics.uptime))
        .replace("{LOAD_AVERAGE}", &get_load_average());

    Ok(Html(rendered))
}
