use crate::models::short::msd::types::MSDShort;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<MSDShort>, SqlxError> {
    let result = sqlx::query_as!(
        MSDShort,
        r#"
        SELECT id, overall, main_pattern
        FROM msd
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
