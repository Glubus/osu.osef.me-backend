use crate::models::extended::beatmap::types::BeatmapExtended;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_all(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<BeatmapExtended>, SqlxError> {
    sqlx::query_as!(
        BeatmapExtended,
        "SELECT * FROM beatmap ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}
