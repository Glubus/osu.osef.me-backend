use crate::models::extended::beatmapset::types::BeatmapsetExtended;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BeatmapsetExtended>, SqlxError> {
    sqlx::query_as!(
        BeatmapsetExtended,
        "SELECT * FROM beatmapset WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_osu_id(
    pool: &PgPool,
    osu_id: i32,
) -> Result<Option<BeatmapsetExtended>, SqlxError> {
    sqlx::query_as!(
        BeatmapsetExtended,
        "SELECT * FROM beatmapset WHERE osu_id = $1",
        osu_id
    )
    .fetch_optional(pool)
    .await
}
