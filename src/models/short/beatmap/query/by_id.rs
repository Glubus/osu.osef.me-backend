use crate::models::short::beatmap::types::BeatmapShort;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BeatmapShort>, SqlxError> {
    sqlx::query_as!(
        BeatmapShort,
        r#"
        SELECT id, osu_id, difficulty, difficulty_rating, mode, status
        FROM beatmap
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<BeatmapShort>, SqlxError> {
    sqlx::query_as!(
        BeatmapShort,
        r#"
        SELECT id, osu_id, difficulty, difficulty_rating, mode, status
        FROM beatmap
        WHERE osu_id = $1
        "#,
        osu_id
    )
    .fetch_optional(pool)
    .await
}
