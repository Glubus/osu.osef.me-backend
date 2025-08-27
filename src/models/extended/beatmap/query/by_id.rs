use crate::models::extended::beatmap::types::BeatmapExtended;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BeatmapExtended>, SqlxError> {
    sqlx::query_as!(
        BeatmapExtended,
        r#"
        SELECT * FROM beatmap WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_beatmapset_id(pool: &PgPool, beatmap_id: i32) -> Result<Option<i32>, SqlxError> {
    let row = sqlx::query!(
        "SELECT beatmapset_id FROM beatmap WHERE id = $1",
        beatmap_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|r| r.beatmapset_id))
}
