use async_trait::async_trait;
use chrono::NaiveDateTime;
use rosu_v2::model::beatmap::BeatmapsetExtended;
use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlxError, PgPool};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Beatmapset {
    pub id: Option<i32>,
    pub osu_id: i32,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub title: String,
    pub title_unicode: Option<String>,
    pub creator: String,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub has_video: bool,
    pub has_storyboard: bool,
    pub is_explicit: bool,
    pub is_featured: bool,
    pub cover_url: Option<String>,
    pub preview_url: Option<String>,
    pub osu_file_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError>;
}

#[async_trait]
impl Insert for Beatmapset {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO beatmapset (
                osu_id, artist, artist_unicode, title, title_unicode, creator, source,
                tags, has_video, has_storyboard, is_explicit, is_featured,
                cover_url, preview_url, osu_file_url
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            ON CONFLICT (osu_id) DO UPDATE SET
                artist = EXCLUDED.artist,
                artist_unicode = EXCLUDED.artist_unicode,
                title = EXCLUDED.title,
                title_unicode = EXCLUDED.title_unicode,
                creator = EXCLUDED.creator,
                source = EXCLUDED.source,
                tags = EXCLUDED.tags,
                has_video = EXCLUDED.has_video,
                has_storyboard = EXCLUDED.has_storyboard,
                is_explicit = EXCLUDED.is_explicit,
                is_featured = EXCLUDED.is_featured,
                cover_url = EXCLUDED.cover_url,
                preview_url = EXCLUDED.preview_url,
                osu_file_url = EXCLUDED.osu_file_url,
                updated_at = now()
            RETURNING id
            "#,
            self.osu_id,
            self.artist,
            self.artist_unicode.as_deref(),
            self.title,
            self.title_unicode.as_deref(),
            self.creator,
            self.source.as_deref(),
            self.tags.as_deref(),
            self.has_video,
            self.has_storyboard,
            self.is_explicit,
            self.is_featured,
            self.cover_url.as_deref(),
            self.preview_url.as_deref(),
            self.osu_file_url.as_deref(),
        );

        let result = query.fetch_one(pool).await?;
        Ok(result.id)
    }
}

impl From<BeatmapsetExtended> for Beatmapset {
    fn from(beatmapset: BeatmapsetExtended) -> Self {
        Self {
            id: None,
            osu_id: beatmapset.mapset_id as i32,
            artist: beatmapset.artist,
            artist_unicode: beatmapset.artist_unicode,
            title: beatmapset.title,
            title_unicode: beatmapset.title_unicode,
            creator: beatmapset.creator_name.to_string(),
            source: Some(beatmapset.source.to_string()),
            tags: None,
            has_video: beatmapset.video,
            has_storyboard: beatmapset.storyboard,
            is_explicit: beatmapset.nsfw,
            is_featured: false,
            cover_url: Some(beatmapset.covers.cover.to_string()),
            preview_url: Some(beatmapset.preview_url),
            osu_file_url: Some(beatmapset.source.to_string()),
            created_at: None,
            updated_at: None,
        }
    }
}

impl Beatmapset {
    /// Insère un beatmapset dans la base de données
    pub async fn insert_into_db(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        self.insert(pool).await
    }

    /// Récupère un beatmapset par son ID
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, SqlxError> {
        let query = sqlx::query_as_unchecked!(
            Beatmapset,
            r#"
            SELECT 
                id, osu_id, artist, artist_unicode, title, title_unicode, creator, source,
                tags, has_video, has_storyboard, is_explicit, is_featured,
                cover_url, preview_url, osu_file_url, created_at, updated_at
            FROM beatmapset 
            WHERE id = $1
            "#,
            id
        );

        query.fetch_optional(pool).await
    }

    /// Récupère un beatmapset par son osu_id
    pub async fn find_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<Self>, SqlxError> {
        let query = sqlx::query_as_unchecked!(
            Beatmapset,
            r#"
            SELECT 
                id, osu_id, artist, artist_unicode, title, title_unicode, creator, source,
                tags, has_video, has_storyboard, is_explicit, is_featured,
                cover_url, preview_url, osu_file_url, created_at, updated_at
            FROM beatmapset 
            WHERE osu_id = $1
            "#,
            osu_id
        );

        query.fetch_optional(pool).await
    }

    /// Vérifie si un beatmapset existe déjà par son osu_id
    pub async fn exists_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<bool, SqlxError> {
        let query = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM beatmapset WHERE osu_id = $1
            ) as exists
            "#,
            osu_id
        );

        let result = query.fetch_one(pool).await?;
        Ok(result.exists.unwrap_or(false))
    }

    /// Recherche des beatmapsets par artiste ou titre
    pub async fn search(
        pool: &PgPool,
        search_term: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>, SqlxError> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let query = sqlx::query_as_unchecked!(
            Beatmapset,
            r#"
            SELECT 
                id, osu_id, artist, artist_unicode, title, title_unicode, creator, source,
                tags, has_video, has_storyboard, is_explicit, is_featured,
                cover_url, preview_url, osu_file_url, created_at, updated_at
            FROM beatmapset 
            WHERE 
                artist ILIKE $1 OR 
                artist_unicode ILIKE $1 OR 
                title ILIKE $1 OR 
                title_unicode ILIKE $1 OR
                creator ILIKE $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            format!("%{}%", search_term),
            limit,
            offset
        );

        query.fetch_all(pool).await
    }

    /// Récupère tous les beatmapsets avec pagination
    pub async fn find_all(
        pool: &PgPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>, SqlxError> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let query = sqlx::query_as_unchecked!(
            Beatmapset,
            r#"
            SELECT 
                id, osu_id, artist, artist_unicode, title, title_unicode, creator, source,
                tags, has_video, has_storyboard, is_explicit, is_featured,
                cover_url, preview_url, osu_file_url, created_at, updated_at
            FROM beatmapset 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        );

        query.fetch_all(pool).await
    }
}
