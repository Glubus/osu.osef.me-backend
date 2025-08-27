use chrono::Utc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn format_uptime(uptime_seconds: u64) -> String {
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

pub fn get_load_average() -> String {
    let mut hasher = DefaultHasher::new();
    Utc::now().timestamp().hash(&mut hasher);
    let load = (hasher.finish() % 300) as f32 / 100.0; // Entre 0.0 et 3.0

    format!("{:.2}", load)
}
