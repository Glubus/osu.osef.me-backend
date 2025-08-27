use crate::models::failed_query::types::FailedQuery;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, hash: &str) -> Result<i32, SqlxError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO failed_query (hash)
        VALUES ($1)
        RETURNING id
        "#,
        hash
    )
    .fetch_one(pool)
    .await?;

    Ok(result.id)
}
