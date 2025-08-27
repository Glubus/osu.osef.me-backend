use sqlx::{Error as SqlxError, PgPool, Row};

pub async fn insert(pool: &PgPool, hash: &str) -> Result<i32, SqlxError> {
    let row = sqlx::query(
        r#"
        INSERT INTO pending_beatmap (hash)
        VALUES ($1)
        ON CONFLICT (hash) DO NOTHING
        RETURNING id
        "#,
    )
    .bind(hash)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.get::<i32, _>("id")).unwrap_or(0))
}
