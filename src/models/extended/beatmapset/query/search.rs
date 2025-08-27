use crate::models::extended::beatmapset::types::BeatmapsetExtended;
use sqlx::{Error as SqlxError, PgPool};

pub async fn search(
    pool: &PgPool,
    term: &str,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<BeatmapsetExtended>, SqlxError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    sqlx::query_as!(
        BeatmapsetExtended,
        r#"
        SELECT * FROM beatmapset
        WHERE artist ILIKE $1 OR artist_unicode ILIKE $1 OR title ILIKE $1 OR title_unicode ILIKE $1 OR creator ILIKE $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        format!("%{}%", term),
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub async fn find_all(
    pool: &PgPool,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<BeatmapsetExtended>, SqlxError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    sqlx::query_as!(
        BeatmapsetExtended,
        "SELECT * FROM beatmapset ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}
