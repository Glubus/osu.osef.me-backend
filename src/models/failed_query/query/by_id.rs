use crate::models::failed_query::FailedQuery;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<FailedQuery>, SqlxError> {
    let row = sqlx::query_as!(
        FailedQuery,
        r#"
        SELECT id, hash, created_at
        FROM failed_query
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
