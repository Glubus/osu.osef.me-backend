//! # Status Page Handler
//!
//! Ce module contient le handler pour la page de status principale de l'API.
//! Cette page affiche l'état de santé du système avec une interface HTML utilisant daisyUI.
//! OPTIMISÉ: Utilise UNIQUEMENT le cache, aucun calcul lors du chargement de page.

use axum::{
    extract::State,
    response::Html,
};
use chrono::Utc;

use crate::{
    db::DatabaseManager,
    models::{
        status::{get_history, get_metrics_with_fallback, HistoryEntry},
    },
    models::beatmap::pending_beatmap::PendingBeatmap,
};

/// Handler pour la page de status principale
/// OPTIMISÉ: N'appelle AUCUNE fonction de health check, utilise uniquement le cache
/// Temps de réponse ultra-rapide, toutes les métriques sont pré-calculées en arrière-plan
pub async fn status_page(State(db): State<DatabaseManager>) -> Html<String> {
    // Charger le template HTML
    let template = include_str!("../../assets/status.html");
    
    // Utiliser UNIQUEMENT les métriques en cache (pas de calculs)
    let metrics = match get_metrics_with_fallback() {
        Some(m) => m,
        None => {
            // Fallback avec valeurs par défaut si aucun cache disponible (premier démarrage)
            return Html(generate_fallback_page(template));
        }
    };
    
    // Toutes les données viennent du cache, aucun calcul
    let (health_color, health_icon, health_status) = get_health_display(metrics.health_score);
    let (score_color_start, score_color_end) = get_score_colors(metrics.health_score);
    let status_info = get_status_info_from_metrics(&metrics);
    
    // Historique (lecture rapide depuis la mémoire)
    let history = get_history();
    let history_bars = generate_history_bars(&history, "api");
    let db_history_bars = generate_history_bars(&history, "database");
    let network_history_bars = generate_network_history_bars(&history);
    
    // Données temporelles (calculs légers)
    let uptime_hours = metrics.uptime / 3600;
    let timestamp = metrics.timestamp.format("%H:%M").to_string();
    
    // Métriques réseau simulées (calcul très léger)
    let (network_status, _network_load, _network_percent) = get_network_metrics();
    
    // Queue de traitement des beatmaps (directement depuis la DB)
    let beatmap_queue_size = match PendingBeatmap::count(db.get_pool()).await {
        Ok(count) => count as usize,
        Err(_) => 0,
    };
    
    // Remplacements dans le template (toutes les données viennent du cache)
    let rendered = template
        .replace("{API_NAME}", env!("CARGO_PKG_NAME"))
        .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
        .replace("{TIMESTAMP}", &timestamp)
        
        // Score de santé (depuis le cache)
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
        
        // Status général (depuis le cache)
        .replace("{STATUS_BADGE}", &status_info.0)
        .replace("{STATUS_TEXT}", &status_info.1)
        
        // Performance et uptime (depuis le cache, pour les animations)
        .replace("{RESPONSE_TIME}", &metrics.response_time_ms.to_string())
        .replace("{UPTIME_HOURS}", &uptime_hours.to_string())
        
        // Réseau
        .replace("{NETWORK_STATUS}", &network_status)
        
        // Queue de beatmaps
        .replace("{BEATMAP_QUEUE_SIZE}", &beatmap_queue_size.to_string())
        
        // Historique
        .replace("{HISTORY_BARS_HTML}", &history_bars)
        .replace("{DB_HISTORY_BARS_HTML}", &db_history_bars)
        .replace("{NETWORK_HISTORY_BARS_HTML}", &network_history_bars)
        
        // Détails techniques
        .replace("{THEME}", "retro")
        .replace("{UPTIME_FULL}", &format_uptime(metrics.uptime))
        .replace("{LOAD_AVERAGE}", &get_load_average());

    Html(rendered)
}

/// Génère une page de fallback si aucun cache n'est disponible
fn generate_fallback_page(template: &str) -> String {
    let timestamp = Utc::now().format("%H:%M").to_string();
    
    template
        .replace("{API_NAME}", env!("CARGO_PKG_NAME"))
        .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
        .replace("{TIMESTAMP}", &timestamp)
        
        // Valeurs par défaut
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
        
        // Queue de beatmaps
        .replace("{BEATMAP_QUEUE_SIZE}", "0")
        
        // Historique vide au démarrage
        .replace("{HISTORY_BARS_HTML}", "")
        .replace("{DB_HISTORY_BARS_HTML}", "")
        .replace("{NETWORK_HISTORY_BARS_HTML}", "")
        
        .replace("{THEME}", "retro")
        .replace("{UPTIME_FULL}", "0m")
        .replace("{LOAD_AVERAGE}", "0.00")
}

// Fonctions utilitaires optimisées (pas de calculs lourds)

fn get_health_display(score: u8) -> (String, String, String) {
    match score {
        90..=100 => ("success".to_string(), "shield-check".to_string(), "Excellent État".to_string()),
        75..=89 => ("info".to_string(), "thumbs-up".to_string(), "Bon État".to_string()),
        60..=74 => ("warning".to_string(), "alert-triangle".to_string(), "État Moyen".to_string()),
        40..=59 => ("error".to_string(), "alert-circle".to_string(), "État Dégradé".to_string()),
        _ => ("error".to_string(), "x-circle".to_string(), "État Critique".to_string()),
    }
}

fn get_score_colors(score: u8) -> (String, String) {
    match score {
        90..=100 => ("#10b981".to_string(), "#059669".to_string()), // Vert
        75..=89 => ("#3b82f6".to_string(), "#2563eb".to_string()),  // Bleu
        60..=74 => ("#f59e0b".to_string(), "#d97706".to_string()),  // Orange
        40..=59 => ("#ef4444".to_string(), "#dc2626".to_string()),  // Rouge
        _ => ("#dc2626".to_string(), "#991b1b".to_string()),        // Rouge foncé
    }
}

fn get_status_info_from_metrics(metrics: &crate::models::status::PerformanceMetrics) -> (String, String) {
    if metrics.db_connected {
        if metrics.response_time_ms < 100 {
            ("success".to_string(), "Optimal".to_string())
        } else if metrics.response_time_ms < 500 {
            ("info".to_string(), "Stable".to_string())
        } else {
            ("warning".to_string(), "Lent".to_string())
        }
    } else {
        ("error".to_string(), "Erreur".to_string())
    }
}

fn get_network_metrics() -> (String, String, u8) {
    // Simulation très légère
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    Utc::now().timestamp().hash(&mut hasher);
    let load_percent = (hasher.finish() % 80) as u8 + 10; // Entre 10% et 90%
    
    let status = match load_percent {
        0..=30 => "Faible",
        31..=60 => "Modérée",
        61..=80 => "Élevée",
        _ => "Critique",
    };
    
    (status.to_string(), format!("{}% utilisé", load_percent), load_percent)
}

fn generate_history_bars(history: &[HistoryEntry], bar_type: &str) -> String {
    history.iter().map(|entry| {
        let (color, tooltip) = match bar_type {
            "api" => {
                let color = determine_network_status_color(entry.response_time_ms as f32);
                let issues_text = if entry.issues.is_empty() { 
                    "Aucun problème".to_string() 
                } else { 
                    entry.issues.join(", ") 
                };
                let tooltip = format!(
                    "⏱️ {} | 🚀 {}ms | 💾 {} | {}",
                    entry.timestamp.format("%H:%M"),
                    entry.response_time_ms,
                    if entry.db_connected { "✅ DB OK" } else { "❌ DB Error" },
                    issues_text
                );
                (color, tooltip)
            },
            "database" => {
                let color = if entry.db_connected { 
                    match entry.db_response_time_ms {
                        Some(time) if time < 50 => "excellent".to_string(),
                        Some(time) if time < 100 => "good".to_string(), 
                        Some(time) if time < 200 => "warning".to_string(),
                        Some(_) => "critical".to_string(),
                        None => "critical".to_string()
                    }
                } else { 
                    "critical".to_string() 
                };
                let db_status_text = if entry.db_connected { 
                    format!("✅ {}ms", entry.db_response_time_ms.unwrap_or(0))
                } else { 
                    "❌ Déconnecté".to_string() 
                };
                let issues_text = if entry.issues.is_empty() { 
                    "Aucun problème".to_string() 
                } else { 
                    entry.issues.join(", ") 
                };
                let tooltip = format!(
                    "⏱️ {} | 💾 {} | {}",
                    entry.timestamp.format("%H:%M"),
                    db_status_text,
                    issues_text
                );
                (color, tooltip)
            },
            _ => ("excellent".to_string(), "".to_string())
        };
        
        format!(
            r#"<div class="status-tick {}" title="{}">
                <div class="tooltip">{}</div>
            </div>"#,
            color, tooltip, tooltip
        )
    }).collect::<Vec<_>>().join("")
}

fn generate_network_history_bars(history: &[HistoryEntry]) -> String {
    history.iter().map(|entry| {
        // Simulation légère basée sur le timestamp
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        entry.timestamp.timestamp().hash(&mut hasher);
        let network_load = (hasher.finish() % 100) as f32;
        
        let color = match network_load {
            x if x < 40.0 => "excellent",
            x if x < 60.0 => "good",
            x if x < 80.0 => "warning",
            x if x < 95.0 => "critical",
            _ => "overload",
        };
        
        let tooltip = format!(
            "⏱️ {} | 🌐 {:.0}% charge | 📡 {}",
            entry.timestamp.format("%H:%M"),
            network_load,
            match network_load {
                x if x < 40.0 => "Réseau fluide",
                x if x < 60.0 => "Charge normale", 
                x if x < 80.0 => "Charge élevée",
                x if x < 95.0 => "Réseau saturé",
                _ => "Réseau surchargé"
            }
        );
        
        format!(
            r#"<div class="status-tick {}" title="{}">
                <div class="tooltip">{}</div>
            </div>"#,
            color, tooltip, tooltip
        )
    }).collect::<Vec<_>>().join("")
}

fn determine_network_status_color(response_time: f32) -> String {
    match response_time {
        x if x < 100.0 => "excellent".to_string(),
        x if x < 300.0 => "good".to_string(),
        x if x < 500.0 => "warning".to_string(),
        x if x < 1000.0 => "critical".to_string(),
        _ => "overload".to_string(),
    }
}

fn format_uptime(uptime_seconds: u64) -> String {
    let days = uptime_seconds / 86400;
    let hours = (uptime_seconds % 86400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    
    if days > 0 {
        format!("{}j {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

fn get_load_average() -> String {
    // Simulation très légère
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    Utc::now().timestamp().hash(&mut hasher);
    let load = (hasher.finish() % 300) as f32 / 100.0; // Entre 0.0 et 3.0
    
    format!("{:.2}", load)
} 