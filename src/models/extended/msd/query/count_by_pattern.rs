use sqlx::{Error as SqlxError, PgPool};
use std::collections::HashMap;
use serde_json::Value;

/// Compte le nombre de beatmaps par pattern en utilisant main_patterns avec rate = 1.0
pub async fn count_beatmaps_by_pattern(pool: &PgPool) -> Result<HashMap<String, u64>, SqlxError> {
    // Requête optimisée avec index sur rate et main_pattern
    // Utiliser LIMIT pour éviter de traiter trop de données
    let rows = sqlx::query!(
        r#"
        SELECT main_pattern, COUNT(*) as count
        FROM msd
        WHERE rate = 1.0 AND main_pattern IS NOT NULL
        GROUP BY main_pattern
        ORDER BY count DESC
        LIMIT 20
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut pattern_counts: HashMap<String, u64> = HashMap::new();
    
    for row in rows {
        if let Some(pattern_json) = row.main_pattern {
            // Parser le JSON string pour extraire les patterns
            if let Ok(json_value) = serde_json::from_str::<Value>(&pattern_json) {
                if let Some(array) = json_value.as_array() {
                    // Prendre le premier pattern du tableau (le plus dominant)
                    if let Some(first_pattern) = array.first() {
                        if let Some(pattern_str) = first_pattern.as_str() {
                            *pattern_counts.entry(pattern_str.to_string()).or_insert(0) += row.count.unwrap_or(0) as u64;
                        }
                    }
                }
            }
        }
    }

    Ok(pattern_counts)
}
