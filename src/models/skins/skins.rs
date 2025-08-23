use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Error as SqlxError, FromRow, Row};
use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Skin {
    pub id: i32,
    pub name: String,
    pub author: String,
    pub version: String,
    pub download_url: String,
    pub download_count: Option<i32>,
    pub cover_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub note_type: String,
    pub tags: Vec<String>,
}

impl Skin {
    /// Créer un nouveau skin
    pub fn new(
        name: String,
        author: String,
        version: String,
        download_url: String,
        note_type: String,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id: -1,
            name,
            author,
            version,
            download_url,
            download_count: None,
            cover_url: None,
            created_at: None,
            note_type,
            tags,
        }
    }

    /// Insérer un skin dans la base de données
    pub async fn insert_into_db(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO skins (name, author, version, download_url, download_count, cover_url, note_type, tags)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            self.name,
            self.author,
            self.version,
            self.download_url,
            self.download_count,
            self.cover_url,
            self.note_type,
            &self.tags
        )
        .fetch_one(pool)
        .await?;

        Ok(result.id)
    }

    /// Mettre à jour un skin existant
    pub async fn update_in_db(&self, pool: &PgPool) -> Result<(), SqlxError> {

        sqlx::query!(
            r#"
            UPDATE skins 
            SET name = $1, author = $2, version = $3, download_url = $4, note_type = $5, tags = $6
            WHERE id = $7
            "#,
            self.name,
            self.author,
            self.version,
            self.download_url,
            self.note_type,
            &self.tags,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Supprimer un skin par ID
    pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<(), SqlxError> {
        sqlx::query!(
            "DELETE FROM skins WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Trouver un skin par ID
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, SqlxError> {
        let result = sqlx::query_as!(
            Self,
            r#"
            SELECT id, name, author, version, download_url, download_count, cover_url, created_at, note_type, tags
            FROM skins
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)  
    }

    /// Trouver tous les skins
    pub async fn find_all(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Self>, SqlxError> {
        let results = sqlx::query_as!(
            Self,
            r#"
            SELECT id, name, author, version, download_url, download_count, cover_url, created_at, note_type, tags
            FROM skins
            ORDER BY created_at DESC
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    /// Trouver les skins par type de note
    pub async fn find_by_note_type(pool: &PgPool, note_type: &str, limit: i64, offset: i64) -> Result<Vec<Self>, SqlxError> {
        let results = sqlx::query_as!(
            Self,
            r#"
            SELECT id, name, author, version, download_url, download_count, cover_url, created_at, note_type, tags
            FROM skins
            WHERE note_type = $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            note_type,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    /// Trouver les skins par tags (recherche partielle)
    pub async fn find_by_tags(pool: &PgPool, tags: &[String], limit: i64, offset: i64) -> Result<Vec<Self>, SqlxError> {
        let results = sqlx::query_as!(
            Self,
            r#"
            SELECT id, name, author, version, download_url, download_count, cover_url, created_at, note_type, tags
            FROM skins
            WHERE tags && $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            tags,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    /// Rechercher les skins par nom ou auteur
    pub async fn search(pool: &PgPool, search_term: &str, limit: i64, offset: i64) -> Result<Vec<Self>, SqlxError> {
        let search_pattern = format!("%{}%", search_term);
        
        let results = sqlx::query_as!(
            Self,
            r#"
            SELECT id, name, author, version, download_url, download_count, cover_url, created_at, note_type, tags
            FROM skins
            WHERE name ILIKE $1 OR author ILIKE $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            search_pattern,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    /// Compter le nombre total de skins
    pub async fn count(pool: &PgPool) -> Result<i64, SqlxError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM skins"
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count.ok_or(SqlxError::RowNotFound)?)
    }

    /// Compter le nombre de skins par type de note
    pub async fn count_by_note_type(pool: &PgPool, note_type: &str) -> Result<i64, SqlxError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM skins WHERE note_type = $1",
            note_type
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count.ok_or(SqlxError::RowNotFound)?)
    }

    /// Compter le nombre de skins par recherche textuelle
    pub async fn count_by_search(pool: &PgPool, search_term: &str) -> Result<i64, SqlxError> {
        let search_pattern = format!("%{}%", search_term);
        
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM skins WHERE name ILIKE $1 OR author ILIKE $1",
            search_pattern
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count.ok_or(SqlxError::RowNotFound)?)
    }

    /// Compter le nombre de skins par tags
    pub async fn count_by_tags(pool: &PgPool, tags: &[String]) -> Result<i64, SqlxError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM skins WHERE tags && $1",
            tags
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count.ok_or(SqlxError::RowNotFound)?)
    }

    /// Rechercher les skins avec filtres multiples
    pub async fn search_by_filters(
        pool: &PgPool,
        note_type: Option<&str>,
        tags: Option<&[String]>,
        search: Option<&str>,
        limit: i64,
        offset: i64,
        order_by: Option<&str>,
    ) -> Result<Vec<Self>, SqlxError> {
        // Construire la requête avec tous les filtres disponibles
        let mut conditions = Vec::new();
        let mut param_count = 0;

        // Filtre par note_type
        if let Some(note_type) = note_type {
            param_count += 1;
            conditions.push(format!("note_type = ${}", param_count));
        }

        // Filtre par tags
        if let Some(tags) = tags {
            param_count += 1;
            conditions.push(format!("tags && ${}", param_count));
        }

        // Filtre par recherche textuelle
        if let Some(search_term) = search {
            param_count += 1;
            conditions.push(format!("(name ILIKE ${} OR author ILIKE ${})", param_count, param_count));
        }

        // Construire la requête SQL
        let mut sql = String::from(
            "SELECT id, name, author, version, download_url, download_count, cover_url, created_at, note_type, tags FROM skins"
        );

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        // Ajouter l'ordre de tri
        match order_by {
            Some("download_count") => {
                sql.push_str(" ORDER BY download_count DESC NULLS LAST, created_at DESC");
            }
            Some("download_count_asc") => {
                sql.push_str(" ORDER BY download_count ASC NULLS LAST, created_at DESC");
            }
            Some("name") => {
                sql.push_str(" ORDER BY name ASC, created_at DESC");
            }
            Some("name_desc") => {
                sql.push_str(" ORDER BY name DESC, created_at DESC");
            }
            Some("author") => {
                sql.push_str(" ORDER BY author ASC, created_at DESC");
            }
            Some("author_desc") => {
                sql.push_str(" ORDER BY author DESC, created_at DESC");
            }
            Some("created_at") => {
                sql.push_str(" ORDER BY created_at ASC");
            }
            Some("created_at_desc") => {
                sql.push_str(" ORDER BY created_at DESC");
            }
            _ => {
                // Ordre par défaut
                sql.push_str(" ORDER BY created_at DESC");
            }
        }

        param_count += 1;
        sql.push_str(&format!(" LIMIT ${}", param_count));
        param_count += 1;
        sql.push_str(&format!(" OFFSET ${}", param_count));

        // Exécuter la requête avec les paramètres
        let mut query_builder = sqlx::query_as::<_, Self>(&sql);
        
        // Ajouter tous les paramètres
        if let Some(note_type) = note_type {
            query_builder = query_builder.bind(note_type);
        }
        if let Some(tags) = tags {
            query_builder = query_builder.bind(tags);
        }
        if let Some(search_term) = search {
            query_builder = query_builder.bind(format!("%{}%", search_term));
        }
        
        // Ajouter limit et offset
        query_builder = query_builder.bind(limit).bind(offset);

        let results = query_builder.fetch_all(pool).await?;
        Ok(results)
    }

    /// Compter les skins avec filtres multiples
    pub async fn count_by_filters(
        pool: &PgPool,
        note_type: Option<&str>,
        tags: Option<&[String]>,
        search: Option<&str>,
    ) -> Result<i64, SqlxError> {
        // Construire la requête avec tous les filtres disponibles
        let mut conditions = Vec::new();

        // Filtre par note_type
        if let Some(note_type) = note_type {
            conditions.push("note_type = $1".to_string());
        }

        // Filtre par tags
        if let Some(_tags) = tags {
            let tag_index = if note_type.is_some() { 2 } else { 1 };
            conditions.push(format!("tags && ${}", tag_index));
        }

        // Filtre par recherche textuelle
        if let Some(_search) = search {
            let search_index = if note_type.is_some() {
                if tags.is_some() { 3 } else { 2 }
            } else {
                if tags.is_some() { 2 } else { 1 }
            };
            conditions.push(format!("(name ILIKE ${} OR author ILIKE ${})", search_index, search_index));
        }

        // Construire la requête SQL
        let mut sql = String::from("SELECT COUNT(*) as count FROM skins");
        
        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        // Exécuter la requête avec les paramètres
        let mut query_builder = sqlx::query(&sql);
        
        // Ajouter tous les paramètres
        if let Some(note_type) = note_type {
            query_builder = query_builder.bind(note_type);
        }
        if let Some(tags) = tags {
            query_builder = query_builder.bind(tags);
        }
        if let Some(search_term) = search {
            query_builder = query_builder.bind(format!("%{}%", search_term));
        }

        let result = query_builder.fetch_one(pool).await?;
        let count: i64 = result.try_get("count")?;
        Ok(count)
    }
}
