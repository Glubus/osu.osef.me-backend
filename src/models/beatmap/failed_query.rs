use chrono::NaiveDateTime;
use sqlx::{Error as SqlxError, PgPool};

#[derive(Debug, Clone)]
pub struct FailedQuery {
    pub id: i32,
    pub hash: String,
    pub created_at: Option<NaiveDateTime>,
}

impl FailedQuery {
    /// Insère une nouvelle entrée failed_query dans la base de données
    pub async fn insert(pool: &PgPool, hash: &str) -> Result<i32, SqlxError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO failed_query (hash)
            VALUES ($1)
            RETURNING id
            "#,
            hash
        )
        .fetch_one(pool)
        .await?;

        Ok(result.id)
    }

    /// Vérifie si un hash existe déjà dans la table failed_query
    pub async fn exists_by_hash(pool: &PgPool, hash: &str) -> Result<bool, SqlxError> {
        let query = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM failed_query WHERE hash = $1
            ) as exists
            "#,
            hash
        );
        let result = query.fetch_one(pool).await?;
        Ok(result.exists.unwrap_or(false))
    }

    /// Récupère toutes les entrées failed_query
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, SqlxError> {
        let rows = sqlx::query_as!(
            Self,
            r#"
            SELECT id, hash, created_at
            FROM failed_query
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    /// Récupère une entrée failed_query par son ID
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, SqlxError> {
        let row = sqlx::query_as!(
            Self,
            r#"
            SELECT id, hash, created_at
            FROM failed_query
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Supprime une entrée failed_query par son hash
    pub async fn delete_by_hash(pool: &PgPool, hash: &str) -> Result<u64, SqlxError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM failed_query
            WHERE hash = $1
            "#,
            hash
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Supprime les entrées failed_query plus anciennes qu'une certaine date
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
}
