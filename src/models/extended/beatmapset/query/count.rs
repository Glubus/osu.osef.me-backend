use sqlx::{Error as SqlxError, PgPool};

/// Compte le nombre total de beatmapsets dans la base de données
pub async fn count_beatmapsets(pool: &PgPool) -> Result<Option<i64>, SqlxError> {
    // Utiliser une estimation rapide basée sur les statistiques de la table
    let row = sqlx::query!(
        r#"
        SELECT COALESCE(
            (SELECT reltuples::bigint FROM pg_class WHERE relname = 'beatmapset'),
            (SELECT COUNT(*) FROM beatmapset)
        ) as count
        "#
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count)
}
