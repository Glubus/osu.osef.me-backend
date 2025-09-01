use crate::models::Filters;
use sqlx::{Error as SqlxError, PgPool, Row};
use super::common::{build_query_with_filters, bind_filter_params, QueryType};

pub async fn count_by_filters(
    pool: &PgPool,
    filters: &Filters,
) -> Result<i64, SqlxError> {

    let query_builder = build_query_with_filters(QueryType::Count, filters);
    let mut query = sqlx::query(&query_builder.query);
    query = bind_filter_params(query, filters);

    // Exécuter la requête et récupérer le total
    let row = query.fetch_one(pool).await?;
    let total: i64 = row.try_get("total")?;
    Ok(total)
}
