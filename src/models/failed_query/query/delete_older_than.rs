use chrono::NaiveDateTime;
use sqlx::{Error as SqlxError, PgPool};

pub async fn delete_older_than(pool: &PgPool, date: NaiveDateTime) -> Result<u64, SqlxError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM failed_query
        WHERE created_at < $1   
        "#,
        date
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
