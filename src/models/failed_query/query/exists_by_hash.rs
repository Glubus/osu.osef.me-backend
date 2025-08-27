use sqlx::{Error as SqlxError, PgPool};

pub async fn exists_by_hash(pool: &PgPool, hash: &str) -> Result<bool, SqlxError> {
    let query = sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM failed_query WHERE hash = $1
        ) as exists
        "#,
        hash
    );
    let result = query.fetch_one(pool).await?;
    Ok(result.exists.unwrap_or(false))
}
