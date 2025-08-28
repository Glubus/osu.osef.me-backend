use crate::models::pending_beatmap::query::*;
use crate::models::pending_beatmap::types::PendingBeatmap;
use sqlx::PgPool;

impl PendingBeatmap {
    pub async fn insert(pool: &PgPool, hash: &str, osu_id: Option<i32>) -> Result<i32, sqlx::Error> {
        println!("DEBUG: Inserting beatmap: {:?}, {:?}", hash, osu_id);
        insert(pool, hash, osu_id).await
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

    pub async fn position_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<i64>, sqlx::Error> {
        position_by_osu_id(pool, osu_id).await
    }
}
