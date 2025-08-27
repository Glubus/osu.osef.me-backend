use crate::models::failed_query::FailedQuery;
use crate::models::failed_query::query::{delete_by_hash, exists_by_hash, find_by_id, insert};
use sqlx::{Error as SqlxError, PgPool};

impl FailedQuery {
    pub async fn insert(pool: &PgPool, hash: &str) -> Result<i32, SqlxError> {
        let result = insert(pool, hash).await?;
        Ok(result)
    }

    pub async fn exists_by_hash(pool: &PgPool, hash: &str) -> Result<bool, SqlxError> {
        let result = exists_by_hash(pool, hash).await?;
        Ok(result)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<FailedQuery, SqlxError> {
        let result = find_by_id(pool, id).await?;
        Ok(result.ok_or(SqlxError::RowNotFound)?)
    }

    pub async fn delete_by_hash(pool: &PgPool, hash: &str) -> Result<u64, SqlxError> {
        let result = delete_by_hash(pool, hash).await?;
        Ok(result)
    }
}
