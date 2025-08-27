use crate::models::extended::msd::query::{
    Insert, find_all_by_beatmap_id, find_by_beatmap_id, find_by_beatmap_id_and_rate, find_by_id,
};
use crate::models::extended::msd::types::MSDExtended;
use sqlx::PgPool;

impl MSDExtended {
    pub async fn insert_into_db(&self, pool: &PgPool) -> Result<i32, sqlx::Error> {
        self.insert(pool).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_beatmap_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        find_by_beatmap_id(pool, beatmap_id).await
    }

    pub async fn find_by_beatmap_id_and_rate(
        pool: &PgPool,
        beatmap_id: i32,
        rate: f64,
    ) -> Result<Option<Self>, sqlx::Error> {
        find_by_beatmap_id_and_rate(pool, beatmap_id, rate).await
    }

    pub async fn find_all_by_beatmap_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_all_by_beatmap_id(pool, beatmap_id).await
    }
}
