use crate::models::extended::beatmapset::query::{
    Insert, exists_by_osu_id, find_all, find_by_id, find_by_osu_id, search,
};
use crate::models::extended::beatmapset::types::BeatmapsetExtended;
use sqlx::PgPool;

impl BeatmapsetExtended {
    pub async fn insert_into_db(&self, pool: &PgPool) -> Result<i32, sqlx::Error> {
        self.insert(pool).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_osu_id(pool, osu_id).await
    }

    pub async fn exists_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<bool, sqlx::Error> {
        exists_by_osu_id(pool, osu_id).await
    }

    pub async fn search(
        pool: &PgPool,
        term: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        search(pool, term, limit, offset).await
    }

    pub async fn find_all(
        pool: &PgPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_all(pool, limit, offset).await
    }
}
