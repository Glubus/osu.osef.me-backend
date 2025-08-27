use chrono::NaiveDateTime;
use rosu_v2::model::beatmap::BeatmapsetExtended as BmsetExtended;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeatmapsetExtended {
    pub id: i32,
    pub osu_id: Option<i32>,
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

impl From<BmsetExtended> for BeatmapsetExtended {
    fn from(b: BmsetExtended) -> Self {
        Self {
            id: 0,
            osu_id: Some(b.mapset_id as i32),
            artist: b.artist,
            artist_unicode: b.artist_unicode,
            title: b.title,
            title_unicode: b.title_unicode,
            creator: b.creator_name.to_string(),
            source: Some(b.source.to_string()),
            tags: None,
            has_video: b.video,
            has_storyboard: b.storyboard,
            is_explicit: b.nsfw,
            is_featured: false,
            cover_url: Some(b.covers.cover.to_string()),
            preview_url: Some(b.preview_url),
            osu_file_url: Some(b.source.to_string()),
            created_at: None,
            updated_at: None,
        }
    }
}
