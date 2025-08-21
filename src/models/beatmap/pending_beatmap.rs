use chrono::NaiveDateTime;
use sqlx::{Error as SqlxError, PgPool, Row};

#[derive(Debug, Clone)]
pub struct PendingBeatmap {
    pub id: i32,
    pub hash: String,
    pub created_at: Option<NaiveDateTime>,
}

impl PendingBeatmap {
    pub async fn insert(pool: &PgPool, hash: &str) -> Result<i32, SqlxError> {
        let row = sqlx::query(
            r#"
            insert into pending_beatmap (hash)
            values ($1)
            on conflict (hash) do nothing
            returning id
            "#,
        )
        .bind(hash)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.get::<i32, _>("id")).unwrap_or(0))
    }

    pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<u64, SqlxError> {
        let result = sqlx::query(
            r#"
            delete from pending_beatmap where id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn count(pool: &PgPool) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            select count(*) from pending_beatmap
            "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    pub async fn oldest(pool: &PgPool) -> Result<Option<Self>, SqlxError> {
        let row = sqlx::query_as!(
            Self,
            r#"
            select id, hash, created_at
            from pending_beatmap
            order by created_at asc, id asc
            limit 1
            "#
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
