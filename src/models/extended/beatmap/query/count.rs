use sqlx::{Error as SqlxError, PgPool};

/// Compte le nombre total de beatmaps dans la base de données
pub async fn count_beatmaps(pool: &PgPool) -> Result<Option<i64>, SqlxError> {
    // Utiliser une estimation rapide basée sur les statistiques de la table
    let row = sqlx::query!(
        r#"
        SELECT COALESCE(
            (SELECT reltuples::bigint FROM pg_class WHERE relname = 'beatmap'),
            (SELECT COUNT(*) FROM beatmap)
        ) as count
        "#
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count)
}

/// Récupère toutes les statistiques en une seule requête optimisée
pub async fn get_all_stats(pool: &PgPool) -> Result<(Option<i64>, Option<i64>, std::collections::HashMap<String, u64>), SqlxError> {
    use std::collections::HashMap;
    use serde_json::Value;

    // Requête unique optimisée qui récupère tout en une fois
    let rows = sqlx::query!(
        r#"
        WITH stats AS (
            SELECT 
                (SELECT COALESCE(reltuples::bigint, 0) FROM pg_class WHERE relname = 'beatmap') as beatmap_count,
                (SELECT COALESCE(reltuples::bigint, 0) FROM pg_class WHERE relname = 'beatmapset') as beatmapset_count
        ),
        pattern_stats AS (
            SELECT main_pattern, COUNT(*) as count
            FROM msd
            WHERE rate = 1.0 AND main_pattern IS NOT NULL
            GROUP BY main_pattern
            ORDER BY count DESC
        )
        SELECT 
            s.beatmap_count,
            s.beatmapset_count,
            p.main_pattern,
            p.count as pattern_count
        FROM stats s
        CROSS JOIN pattern_stats p
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut beatmap_count = None;
    let mut beatmapset_count = None;
    let mut patterns: HashMap<String, u64> = HashMap::new();

    for row in rows {
        beatmap_count = Some(row.beatmap_count.unwrap_or(0));
        beatmapset_count = Some(row.beatmapset_count.unwrap_or(0));
        
        if let Some(pattern_json) = row.main_pattern {
            if let Ok(json_value) = serde_json::from_str::<Value>(&pattern_json) {
                if let Some(array) = json_value.as_array() {
                    if let Some(first_pattern) = array.first() {
                        if let Some(pattern_str) = first_pattern.as_str() {
                            *patterns.entry(pattern_str.to_string()).or_insert(0) += row.pattern_count.unwrap_or(0) as u64;
                        }
                    }
                }
            }
        }
    }

    Ok((beatmap_count, beatmapset_count, patterns))
}
