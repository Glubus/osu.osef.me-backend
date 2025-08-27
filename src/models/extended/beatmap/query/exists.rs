use sqlx::{Error as SqlxError, PgPool};

pub async fn exists_by_checksum(pool: &PgPool, checksum: &str) -> Result<bool, SqlxError> {
    let row = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM beatmap WHERE file_md5 = $1) as exists",
        checksum
    )
    .fetch_one(pool)
    .await?;
    Ok(row.exists.unwrap_or(false))
}
