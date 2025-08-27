use sqlx::{Error as SqlxError, PgPool};

pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<u64, SqlxError> {
    let result = sqlx::query(r#"DELETE FROM pending_beatmap WHERE id = $1"#)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

pub async fn delete_by_hash(pool: &PgPool, hash: &str) -> Result<u64, SqlxError> {
    let result = sqlx::query(r#"DELETE FROM pending_beatmap WHERE hash = $1"#)
        .bind(hash)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
