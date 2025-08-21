use crate::helpers::beatmap::{build_file_path, rank_status_to_string};
use crate::helpers::common::{from_f32, from_f64};
use crate::models::beatmap::beatmapset::Beatmapset;
use crate::models::ratings::msd::MSD;
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use rosu_v2::model::beatmap::BeatmapExtended;
use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlxError, PgPool, Row};

// Structures courtes pour les listes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapShort {
    pub id: Option<i32>,
    pub osu_id: i32,
    pub difficulty: String,
    pub difficulty_rating: BigDecimal,
    pub mode: i32,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapsetShort {
    pub id: Option<i32>,
    pub osu_id: i32,
    pub artist: String,
    pub title: String,
    pub creator: String,
    pub cover_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MSDShort {
    pub id: Option<i32>,
    pub overall: Option<BigDecimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapWithMSDShort {
    pub beatmap: Option<BeatmapShort>,
    pub beatmapset: Option<BeatmapsetShort>,
    pub msd: Option<MSDShort>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Filters {
    pub search_term: Option<String>,
    pub overall_min: Option<f64>,
    pub overall_max: Option<f64>,
    pub selected_pattern: Option<String>,
    pub pattern_min: Option<f64>,
    pub pattern_max: Option<f64>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Beatmap {
    pub id: Option<i32>,
    pub osu_id: i32,
    pub beatmapset_id: Option<i32>,
    pub difficulty: String,
    pub difficulty_rating: BigDecimal,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub max_combo: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: BigDecimal,
    pub cs: BigDecimal,
    pub ar: BigDecimal,
    pub od: BigDecimal,
    pub hp: BigDecimal,
    pub mode: i32,
    pub status: String,
    pub file_md5: String,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError>;
}

#[async_trait]
impl Insert for Beatmap {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO beatmap (
                osu_id, beatmapset_id, difficulty, difficulty_rating, 
                count_circles, count_sliders, count_spinners, max_combo,
                drain_time, total_time, bpm, cs, ar, od, hp, mode, 
                status, file_md5, file_path
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING id
            "#,
            self.osu_id,
            self.beatmapset_id,
            self.difficulty,
            self.difficulty_rating,
            self.count_circles,
            self.count_sliders,
            self.count_spinners,
            self.max_combo,
            self.drain_time,
            self.total_time,
            self.bpm,
            self.cs,
            self.ar,
            self.od,
            self.hp,
            self.mode,
            self.status,
            self.file_md5,
            self.file_path
        );

        let result = query.fetch_one(pool).await?;
        Ok(result.id)
    }
}

impl From<BeatmapExtended> for Beatmap {
    fn from(beatmap: BeatmapExtended) -> Self {
        Self {
            id: None,
            beatmapset_id: None,
            osu_id: beatmap.map_id as i32,
            difficulty: beatmap.version,
            difficulty_rating: from_f32(beatmap.stars),
            count_circles: beatmap.count_circles as i32,
            count_sliders: beatmap.count_sliders as i32,
            count_spinners: beatmap.count_spinners as i32,
            max_combo: beatmap.max_combo.unwrap_or(0) as i32,
            drain_time: beatmap.seconds_drain as i32,
            total_time: beatmap.seconds_total as i32,
            bpm: from_f32(beatmap.bpm),
            cs: from_f32(beatmap.cs),
            ar: from_f32(beatmap.ar),
            od: from_f32(beatmap.od),
            hp: from_f32(beatmap.hp),
            mode: beatmap.mode as i32,
            status: rank_status_to_string(&beatmap.status),
            file_md5: beatmap.checksum.unwrap_or_default(),
            file_path: build_file_path(beatmap.map_id),
            created_at: None,
            updated_at: None,
        }
    }
}

impl Beatmap {
    /// Insère un beatmap dans la base de données
    pub async fn insert_into_db(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        self.insert(pool).await
    }

    /// Récupère un beatmap par son ID
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, SqlxError> {
        let query = sqlx::query_as_unchecked!(
            Beatmap,
            r#"
            SELECT 
                id, osu_id, beatmapset_id, difficulty, difficulty_rating,
                count_circles, count_sliders, count_spinners, max_combo,
                drain_time, total_time, bpm, cs, ar, od, hp, mode,
                status, file_md5, file_path, created_at, updated_at
            FROM beatmap 
            WHERE id = $1
            "#,
            id
        );

        query.fetch_optional(pool).await
    }

    /// Vérifie si un beatmap existe déjà par son file_md5 (checksum)
    pub async fn exists_by_checksum(pool: &PgPool, checksum: &str) -> Result<bool, SqlxError> {
        let query = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM beatmap WHERE file_md5 = $1
            ) as exists
            "#,
            checksum
        );

        let result = query.fetch_one(pool).await?;
        Ok(result.exists.unwrap_or(false))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapWithMSD {
    pub beatmap: Option<Beatmap>,
    pub beatmapset: Option<Beatmapset>,
    pub msd: Option<MSD>,
}

impl BeatmapWithMSD {
    pub async fn find_by_beatmap_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Option<Self>, SqlxError> {
        let beatmap = Beatmap::find_by_id(pool, beatmap_id).await?;
        let msd = MSD::find_by_beatmap_id(pool, beatmap_id).await?;

        let beatmapset = if let Some(ref beatmap) = beatmap {
            if let Some(beatmapset_id) = beatmap.beatmapset_id {
                Beatmapset::find_by_id(pool, beatmapset_id).await?
            } else {
                None
            }
        } else {
            None
        };

        Ok(Some(Self {
            beatmap,
            beatmapset,
            msd,
        }))
    }
}

impl BeatmapWithMSDShort {
    pub async fn find_by_filters(pool: &PgPool, filters: &Filters) -> Result<Vec<Self>, SqlxError> {
        let mut query = String::from(
            "SELECT 
                b.id, b.osu_id, b.difficulty, b.difficulty_rating, b.mode, b.status,
                m.id as msd_id, m.overall,
                bs.id as beatmapset_id, bs.osu_id as beatmapset_osu_id, bs.artist, bs.title, bs.creator, bs.cover_url
             FROM beatmap b 
             JOIN msd m ON b.id = m.beatmap_id
             LEFT JOIN beatmapset bs ON b.beatmapset_id = bs.id"
        );

        let mut conditions: Vec<String> = Vec::new();
        let mut param_count = 0;

        if let Some(search_term) = &filters.search_term {
            if !search_term.is_empty() {
                param_count += 1;
                conditions.push(format!(
                    "(b.difficulty ILIKE ${} OR b.status ILIKE ${})",
                    param_count, param_count
                ));
            }
        }

        if let Some(_overall_min) = filters.overall_min {
            param_count += 1;
            conditions.push(format!("m.overall >= ${}", param_count));
        }

        if let Some(_overall_max) = filters.overall_max {
            param_count += 1;
            conditions.push(format!("m.overall <= ${}", param_count));
        }

        if let Some(pattern) = &filters.selected_pattern {
            if !pattern.is_empty() {
                if let Some(_pattern_min) = filters.pattern_min {
                    param_count += 1;
                    conditions.push(format!("m.{} >= ${}", pattern, param_count));
                }
                if let Some(_pattern_max) = filters.pattern_max {
                    param_count += 1;
                    conditions.push(format!("m.{} <= ${}", pattern, param_count));
                }

                param_count += 1;
                conditions.push(format!("m.main_pattern ILIKE ${}", param_count));
            }
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        // Calculer la pagination
        let per_page = filters.per_page.unwrap_or(30);
        let page = filters.page.unwrap_or(1);
        let offset = (page - 1) * per_page;

        param_count += 1;
        query.push_str(&format!(" ORDER BY b.id LIMIT ${}", param_count));
        param_count += 1;
        query.push_str(&format!(" OFFSET ${}", param_count));

        // Construire la requête avec les paramètres appropriés
        let mut query_builder = sqlx::query(&query);

        if let Some(search_term) = &filters.search_term {
            if !search_term.is_empty() {
                let search_pattern = format!("%{}%", search_term);
                query_builder = query_builder.bind(search_pattern);
            }
        }

        if let Some(overall_min) = filters.overall_min {
            query_builder = query_builder.bind(from_f64(overall_min));
        }

        if let Some(overall_max) = filters.overall_max {
            query_builder = query_builder.bind(from_f64(overall_max));
        }

        if let Some(pattern) = &filters.selected_pattern {
            if !pattern.is_empty() {
                if let Some(pattern_min) = filters.pattern_min {
                    query_builder = query_builder.bind(from_f64(pattern_min));
                }
                if let Some(pattern_max) = filters.pattern_max {
                    query_builder = query_builder.bind(from_f64(pattern_max));
                }

                let pattern_search = format!("%\"{}\"%", pattern);
                query_builder = query_builder.bind(pattern_search);
            }
        }

        query_builder = query_builder.bind(per_page as i64).bind(offset as i64);

        // Exécuter la requête
        let rows = query_builder.fetch_all(pool).await?;

        let mut results = Vec::new();
        for row in rows {
            let beatmap = BeatmapShort {
                id: row.try_get(0)?,
                osu_id: row.try_get(1)?,
                difficulty: row.try_get(2)?,
                difficulty_rating: row.try_get(3)?,
                mode: row.try_get(4)?,
                status: row.try_get(5)?,
            };

            let msd = MSDShort {
                id: row.try_get(6)?,
                overall: row.try_get(7)?,
            };

            // Construire le beatmapset si les données sont disponibles
            let beatmapset = if let Ok(beatmapset_id) = row.try_get::<Option<i32>, _>(8) {
                if beatmapset_id.is_some() {
                    Some(BeatmapsetShort {
                        id: row.try_get(8)?,
                        osu_id: row.try_get(9)?,
                        artist: row.try_get(10)?,
                        title: row.try_get(11)?,
                        creator: row.try_get(12)?,
                        cover_url: row.try_get(13)?,
                    })
                } else {
                    None
                }
            } else {
                None
            };

            results.push(BeatmapWithMSDShort {
                beatmap: Some(beatmap),
                beatmapset,
                msd: Some(msd),
            });
        }

        Ok(results)
    }

    pub async fn count_by_filters(pool: &PgPool, filters: &Filters) -> Result<usize, SqlxError> {
        let mut query = String::from(
            "SELECT COUNT(*) as total
             FROM beatmap b 
             JOIN msd m ON b.id = m.beatmap_id
             LEFT JOIN beatmapset bs ON b.beatmapset_id = bs.id",
        );

        let mut conditions: Vec<String> = Vec::new();
        let mut param_count = 0;

        if let Some(search_term) = &filters.search_term {
            if !search_term.is_empty() {
                param_count += 1;
                conditions.push(format!(
                    "(b.difficulty ILIKE ${} OR b.status ILIKE ${})",
                    param_count, param_count
                ));
            }
        }

        if let Some(_overall_min) = filters.overall_min {
            param_count += 1;
            conditions.push(format!("m.overall >= ${}", param_count));
        }

        if let Some(_overall_max) = filters.overall_max {
            param_count += 1;
            conditions.push(format!("m.overall <= ${}", param_count));
        }

        if let Some(pattern) = &filters.selected_pattern {
            if !pattern.is_empty() {
                if let Some(_pattern_min) = filters.pattern_min {
                    param_count += 1;
                    conditions.push(format!("m.{} >= ${}", pattern, param_count));
                }
                if let Some(_pattern_max) = filters.pattern_max {
                    param_count += 1;
                    conditions.push(format!("m.{} <= ${}", pattern, param_count));
                }

                param_count += 1;
                conditions.push(format!("m.main_pattern ILIKE ${}", param_count));
            }
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        let mut query_builder = sqlx::query(&query);

        if let Some(search_term) = &filters.search_term {
            if !search_term.is_empty() {
                let search_pattern = format!("%{}%", search_term);
                query_builder = query_builder.bind(search_pattern);
            }
        }

        if let Some(overall_min) = filters.overall_min {
            query_builder = query_builder.bind(from_f64(overall_min));
        }

        if let Some(overall_max) = filters.overall_max {
            query_builder = query_builder.bind(from_f64(overall_max));
        }

        if let Some(pattern) = &filters.selected_pattern {
            if !pattern.is_empty() {
                if let Some(pattern_min) = filters.pattern_min {
                    query_builder = query_builder.bind(from_f64(pattern_min));
                }
                if let Some(pattern_max) = filters.pattern_max {
                    query_builder = query_builder.bind(from_f64(pattern_max));
                }

                let pattern_search = format!("%\"{}\"%", pattern);
                query_builder = query_builder.bind(pattern_search);
            }
        }

        let row = query_builder.fetch_one(pool).await?;
        let total: i64 = row.try_get(0)?;

        Ok(total as usize)
    }
}
