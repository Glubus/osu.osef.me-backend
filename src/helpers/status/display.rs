use crate::services::status::PerformanceMetrics;

pub fn get_health_display(score: u8) -> (String, String, String) {
    match score {
        90..=100 => (
            "success".into(),
            "shield-check".into(),
            "Excellent État".into(),
        ),
        75..=89 => ("info".into(), "thumbs-up".into(), "Bon État".into()),
        60..=74 => (
            "warning".into(),
            "alert-triangle".into(),
            "État Moyen".into(),
        ),
        40..=59 => ("error".into(), "alert-circle".into(), "État Dégradé".into()),
        _ => ("error".into(), "x-circle".into(), "État Critique".into()),
    }
}

pub fn get_score_colors(score: u8) -> (String, String) {
    match score {
        90..=100 => ("#10b981".into(), "#059669".into()), // Vert
        75..=89 => ("#3b82f6".into(), "#2563eb".into()),  // Bleu
        60..=74 => ("#f59e0b".into(), "#d97706".into()),  // Orange
        40..=59 => ("#ef4444".into(), "#dc2626".into()),  // Rouge
        _ => ("#dc2626".into(), "#991b1b".into()),        // Rouge foncé
    }
}

pub fn get_status_info_from_metrics(metrics: &PerformanceMetrics) -> (String, String) {
    if metrics.db_connected {
        if metrics.response_time_ms < 100 {
            ("success".into(), "Optimal".into())
        } else if metrics.response_time_ms < 500 {
            ("info".into(), "Stable".into())
        } else {
            ("warning".into(), "Lent".into())
        }
    } else {
        ("error".into(), "Erreur".into())
    }
}
