use crate::models::pending_beatmap::PendingBeatmap;
use sqlx::{Error as SqlxError, PgPool};

pub async fn position_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<i64>, SqlxError> {
    let position = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT position
        FROM (
            SELECT 
                osu_id,
                ROW_NUMBER() OVER (ORDER BY created_at ASC, id ASC) as position
            FROM pending_beatmap
        ) ranked
        WHERE osu_id = $1
        "#
    )
    .bind(osu_id)
    .fetch_optional(pool)
    .await?;

    Ok(position)
}
