use crate::models::short::beatmapset::types::BeatmapsetShort;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_osu_id(
    pool: &PgPool,
    osu_id: i32,
) -> Result<Option<BeatmapsetShort>, SqlxError> {
    let result = sqlx::query_as!(
        BeatmapsetShort,
        r#"
        SELECT id, osu_id, artist, title, creator, cover_url
        FROM beatmapset
        WHERE osu_id = $1
        "#,
        osu_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
