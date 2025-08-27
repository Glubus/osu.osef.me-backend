use crate::models::pending_beatmap::query::*;
use crate::models::pending_beatmap::types::PendingBeatmap;
use sqlx::PgPool;

impl PendingBeatmap {
    pub async fn insert(pool: &PgPool, hash: &str) -> Result<i32, sqlx::Error> {
        insert(pool, hash).await
    }

    pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        delete_by_id(pool, id).await
    }

    pub async fn delete_by_hash(pool: &PgPool, hash: &str) -> Result<u64, sqlx::Error> {
        delete_by_hash(pool, hash).await
    }

    pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
        count(pool).await
    }

    pub async fn oldest(pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        oldest(pool).await
    }

    pub async fn bulk_insert(pool: &PgPool, hashes: &[String]) -> Result<usize, sqlx::Error> {
        bulk_insert(pool, hashes).await
    }
}
