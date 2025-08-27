use crate::models::short::beatmapset::types::BeatmapsetShort;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BeatmapsetShort>, SqlxError> {
    let result = sqlx::query_as!(
        BeatmapsetShort,
        r#"
        SELECT id, osu_id, artist, title, creator, cover_url
        FROM beatmapset
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
