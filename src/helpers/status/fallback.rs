use chrono::Utc;

pub fn generate_fallback_page(template: &str) -> String {
    let timestamp = Utc::now().format("%H:%M").to_string();

    template
        .replace("{API_NAME}", env!("CARGO_PKG_NAME"))
        .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
        .replace("{TIMESTAMP}", &timestamp)
        .replace("{HEALTH_SCORE}", "75")
        .replace("{HEALTH_COLOR}", "info")
        .replace("{HEALTH_ICON}", "activity")
        .replace("{HEALTH_STATUS}", "Initialisation...")
        .replace("{SCORE_COLOR_START}", "#3b82f6")
        .replace("{SCORE_COLOR_END}", "#2563eb")
        .replace("{CPU_SCORE}", "20")
        .replace("{MEMORY_SCORE}", "20")
        .replace("{PERF_SCORE}", "20")
        .replace("{NETWORK_SCORE}", "15")
        .replace("{STATUS_BADGE}", "info")
        .replace("{STATUS_TEXT}", "Démarrage")
        .replace("{RESPONSE_TIME}", "50")
        .replace("{UPTIME_HOURS}", "0")
        .replace("{NETWORK_STATUS}", "Initialisation")
        .replace("{HISTORY_BARS_HTML}", "")
        .replace("{DB_HISTORY_BARS_HTML}", "")
        .replace("{NETWORK_HISTORY_BARS_HTML}", "")
        .replace("{THEME}", "retro")
        .replace("{UPTIME_FULL}", "0m")
        .replace("{LOAD_AVERAGE}", "0.00")
}
