use crate::services::status::HistoryEntry;

pub fn generate_history_bars(history: &[HistoryEntry], bar_type: &str) -> String {
    history
        .iter()
        .map(|entry| {
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
                        if entry.db_connected {
                            "✅ DB OK"
                        } else {
                            "❌ DB Error"
                        },
                        issues_text
                    );
                    (color, tooltip)
                }
                "database" => {
                    let color = if entry.db_connected {
                        match entry.db_response_time_ms {
                            Some(time) if time < 50 => "excellent".to_string(),
                            Some(time) if time < 100 => "good".to_string(),
                            Some(time) if time < 200 => "warning".to_string(),
                            Some(_) => "critical".to_string(),
                            None => "critical".to_string(),
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
                }
                _ => ("excellent".to_string(), "".to_string()),
            };

            format!(
                r#"<div class="status-tick {}" title="{}">
                <div class="tooltip">{}</div>
            </div>"#,
                color, tooltip, tooltip
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn generate_network_history_bars(history: &[HistoryEntry]) -> String {
    history
        .iter()
        .map(|entry| {
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
                    _ => "Réseau surchargé",
                }
            );

            format!(
                r#"<div class="status-tick {}" title="{}">
                <div class="tooltip">{}</div>
            </div>"#,
                color, tooltip, tooltip
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

fn determine_network_status_color(response_time: f32) -> String {
    match response_time {
        x if x < 100.0 => "excellent".into(),
        x if x < 300.0 => "good".into(),
        x if x < 500.0 => "warning".into(),
        x if x < 1000.0 => "critical".into(),
        _ => "overload".into(),
    }
}
