use sqlx::{Error as SqlxError, PgPool};

pub async fn delete_by_hash(pool: &PgPool, hash: &str) -> Result<u64, SqlxError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM failed_query
        WHERE hash = $1
        "#,
        hash
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
