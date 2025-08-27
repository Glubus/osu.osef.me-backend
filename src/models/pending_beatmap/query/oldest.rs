use crate::models::pending_beatmap::types::PendingBeatmap;
use sqlx::{Error as SqlxError, PgPool};

pub async fn oldest(pool: &PgPool) -> Result<Option<PendingBeatmap>, SqlxError> {
    let row = sqlx::query_as!(
        PendingBeatmap,
        r#"
        SELECT id, hash, created_at
        FROM pending_beatmap
        ORDER BY created_at ASC, id ASC
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
