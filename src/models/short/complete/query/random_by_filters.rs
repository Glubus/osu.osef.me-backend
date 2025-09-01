use crate::models::short::complete::types::BeatmapsetCompleteShort;
use crate::models::Filters;
use sqlx::{Error as SqlxError, PgPool};
use super::common::{build_query_with_filters, bind_filter_params, QueryType, map_rows_to_beatmapsets};

pub async fn random_by_filters(
    pool: &PgPool,
    filters: &Filters,
) -> Result<Vec<BeatmapsetCompleteShort>, SqlxError> {

    let query_builder = build_query_with_filters(QueryType::Random, filters);
    let mut query = sqlx::query(&query_builder.query);
    query = bind_filter_params(query, filters);
    let rows = query.fetch_all(pool).await?;

    Ok(map_rows_to_beatmapsets(rows))
}




