use crate::models::extended::beatmapset::types::BeatmapsetExtended;
use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool};

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError>;
}

#[async_trait]
impl Insert for BeatmapsetExtended {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO beatmapset (
                osu_id, artist, artist_unicode, title, title_unicode, creator, source,
                tags, has_video, has_storyboard, is_explicit, is_featured,
                cover_url, preview_url, osu_file_url
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
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
            self.osu_file_url.as_deref()
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }
}
