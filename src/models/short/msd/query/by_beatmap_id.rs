use crate::models::short::msd::types::MSDShort;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_beatmap_id(
    pool: &PgPool,
    beatmap_id: i32,
) -> Result<Vec<MSDShort>, SqlxError> {
    let result = sqlx::query_as!(
        MSDShort,
        r#"
        SELECT id, overall, main_pattern
        FROM msd
        WHERE beatmap_id = $1
        ORDER BY id ASC
        "#,
        beatmap_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
