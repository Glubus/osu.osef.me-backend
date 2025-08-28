use sqlx::{Error as SqlxError, PgPool, Row};

pub async fn insert(pool: &PgPool, hash: &str, osu_id: Option<i32>) -> Result<i32, SqlxError> {
    let row = sqlx::query(
        r#"
        INSERT INTO pending_beatmap (hash, osu_id)
        VALUES ($1, $2)
        ON CONFLICT (hash) DO NOTHING
        RETURNING id
        "#,
    )
    .bind(hash)
    .bind(osu_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.get::<i32, _>("id")).unwrap_or(0))
}
