use chrono::Utc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn get_network_metrics() -> (String, String, u8) {
    let mut hasher = DefaultHasher::new();
    Utc::now().timestamp().hash(&mut hasher);
    let load_percent = (hasher.finish() % 80) as u8 + 10; // Entre 10% et 90%

    let status = match load_percent {
        0..=30 => "Faible",
        31..=60 => "Modérée",
        61..=80 => "Élevée",
        _ => "Critique",
    };

    (
        status.into(),
        format!("{}% utilisé", load_percent),
        load_percent,
    )
}
