use crate::models::extended::msd::types::MSDExtended;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<MSDExtended>, SqlxError> {
    sqlx::query_as!(
        MSDExtended,
        r#"
        SELECT * FROM msd WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
