use crate::models::short::complete::query::{count_by_filters, find_by_beatmapset_id, find_by_beatmapset_osu_id, find_by_filters};
use crate::models::short::complete::types::{BeatmapCompleteShort, BeatmapsetCompleteShort};
use crate::models::Filters;
use sqlx::PgPool;

impl BeatmapCompleteShort {
    pub async fn find_by_beatmapset_id(
        pool: &PgPool,
        beatmapset_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_by_beatmapset_id(pool, beatmapset_id).await
    }
}

impl BeatmapsetCompleteShort {
    pub async fn find_by_beatmapset_osu_id(
        pool: &PgPool,
        beatmapset_osu_id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        find_by_beatmapset_osu_id(pool, beatmapset_osu_id).await
    }

    pub async fn find_by_filters(
        pool: &PgPool,
        filters: &Filters,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_by_filters(pool, filters).await
    }

    pub async fn count_by_filters(
        pool: &PgPool,
        filters: &Filters,
    ) -> Result<i64, sqlx::Error> {
        count_by_filters(pool, filters).await
    }
}
