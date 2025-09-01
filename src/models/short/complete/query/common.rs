use crate::models::Filters;
use crate::helpers::common::from_f64;
use sqlx::postgres::PgArguments;
use crate::models::short::complete::types::{BeatmapCompleteShort, BeatmapsetCompleteShort};
use crate::models::short::msd::MSDShort;
use crate::models::short::beatmap::BeatmapShort;
use crate::models::short::beatmapset::BeatmapsetShort;
use std::collections::HashMap;
use sqlx::Row;

/// Structure contenant une requête SQL et le nombre de paramètres
pub struct QueryBuilder {
    pub query: String,
    pub param_count: usize,
}

/// Options pour la construction de requête
#[derive(Clone)]
pub enum QueryType {
    /// Requête SELECT complète avec colonnes
    Select,
    /// Requête COUNT pour compter les résultats
    Count,
    /// Requête SELECT avec ORDER BY RANDOM() et LIMIT
    Random,
}

/// Construit la base de la requête selon le type
pub fn build_base_query(query_type: &QueryType) -> String {
    match query_type {
        QueryType::Select => String::from(
            r#"
        SELECT 
            bs.id as beatmapset_id, bs.osu_id as beatmapset_osu_id, bs.artist, bs.title, bs.creator, bs.cover_url,
            b.id as beatmap_id, b.osu_id as beatmap_osu_id, b.difficulty, b.difficulty_rating, b.mode, b.status,
            m.id as msd_id, m.overall, m.main_pattern
        FROM beatmapset bs
        LEFT JOIN beatmap b ON bs.id = b.beatmapset_id
        LEFT JOIN msd m ON b.id = m.beatmap_id
            "#
        ),
        QueryType::Count => String::from(
            r#"
        SELECT COUNT(DISTINCT bs.id) as total
        FROM beatmapset bs
        LEFT JOIN beatmap b ON bs.id = b.beatmapset_id
        LEFT JOIN msd m ON b.id = m.beatmap_id
            "#
        ),
        QueryType::Random => String::from(
            r#"
        SELECT 
            bs.id as beatmapset_id, bs.osu_id as beatmapset_osu_id, bs.artist, bs.title, bs.creator, bs.cover_url,
            b.id as beatmap_id, b.osu_id as beatmap_osu_id, b.difficulty, b.difficulty_rating, b.mode, b.status,
            m.id as msd_id, m.overall, m.main_pattern
        FROM beatmapset bs
        LEFT JOIN beatmap b ON bs.id = b.beatmapset_id
        LEFT JOIN msd m ON b.id = m.beatmap_id
            "#
        ),
    }
}

/// Construit les conditions WHERE basées sur les filtres
pub fn build_where_conditions(filters: &Filters) -> (Vec<String>, usize) {
    let mut conditions: Vec<String> = Vec::new();
    let mut param_count = 0;

    // Recherche par terme
    if let Some(search_term) = &filters.search_term {
        if !search_term.is_empty() {
            param_count += 1;
            conditions.push(format!(
                "(b.difficulty ILIKE ${} OR b.status ILIKE ${} OR bs.artist ILIKE ${} OR bs.title ILIKE ${} OR bs.creator ILIKE ${})",
                param_count, param_count, param_count, param_count, param_count
            ));
        }
    }

    // Filtre par overall min/max
    if let Some(overall_min) = filters.overall_min {
        param_count += 1;
        conditions.push(format!("m.overall >= ${}", param_count));
    }

    if let Some(overall_max) = filters.overall_max {
        param_count += 1;
        conditions.push(format!("m.overall <= ${}", param_count));
    }

    // Filtre par pattern
    if let Some(pattern) = &filters.selected_pattern {
        let column_name = pattern.as_column_name();
        
        if !column_name.is_empty() {
            if let Some(pattern_min) = filters.pattern_min {
                param_count += 1;
                conditions.push(format!("m.{} >= ${}", column_name, param_count));
            }
            if let Some(pattern_max) = filters.pattern_max {
                param_count += 1;
                conditions.push(format!("m.{} <= ${}", column_name, param_count));
            }

            param_count += 1;
            conditions.push(format!("m.main_pattern ILIKE ${}", param_count));
        }
    }

    // Filtre par BPM
    if let Some(_bpm_min) = filters.bpm_min {
        param_count += 1;
        conditions.push(format!("b.bpm >= ${}", param_count));
    }

    if let Some(_bpm_max) = filters.bpm_max {
        param_count += 1;
        conditions.push(format!("b.bpm <= ${}", param_count));
    }

    // Filtre par temps total
    if let Some(_total_time_min) = filters.total_time_min {
        param_count += 1;
        conditions.push(format!("b.total_time >= ${}", param_count));
    }

    if let Some(_total_time_max) = filters.total_time_max {
        param_count += 1;
        conditions.push(format!("b.total_time <= ${}", param_count));
    }

    // Condition par défaut pour le rate
    conditions.push("m.rate = 1.0".to_string());

    (conditions, param_count)
}

/// Construit une requête complète selon le type et les filtres
pub fn build_query_with_filters(query_type: QueryType, filters: &Filters) -> QueryBuilder {
    let mut query = build_base_query(&query_type);
    let (conditions, mut param_count) = build_where_conditions(filters);

    // Ajouter les conditions WHERE
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    // Ajouter ORDER BY et LIMIT selon le type de requête
    match &query_type {
        QueryType::Select => {
            query.push_str(" ORDER BY bs.id, b.id");
            
            // Ajouter la pagination
            let per_page = filters.per_page.unwrap_or(10);
            let page = filters.page.unwrap_or(1);
            let _offset = (page - 1) * per_page;
            
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
            param_count += 1;
            query.push_str(&format!(" OFFSET ${}", param_count));
        },
        QueryType::Random => {
            query.push_str(" ORDER BY RANDOM()");
            query.push_str(" LIMIT 10");
        },
        QueryType::Count => {
            // Pas d'ORDER BY ou LIMIT pour COUNT
        }
    }

    QueryBuilder { query, param_count }
}

/// Bind les paramètres de filtres à une requête SQL
pub fn bind_filter_params<'q>(
    mut query_builder: sqlx::query::Query<'q, sqlx::Postgres, PgArguments>, 
    filters: &Filters
) -> sqlx::query::Query<'q, sqlx::Postgres, PgArguments> {
    
    // Bind search term
    if let Some(search_term) = &filters.search_term {
        if !search_term.is_empty() {
            let search_pattern = format!("%{}%", search_term);
            query_builder = query_builder.bind(search_pattern);
        }
    }

    // Bind overall min/max
    if let Some(overall_min) = filters.overall_min {
        query_builder = query_builder.bind(from_f64(overall_min));
    }

    if let Some(overall_max) = filters.overall_max {
        query_builder = query_builder.bind(from_f64(overall_max));
    }

    // Bind pattern filters
    if let Some(pattern) = &filters.selected_pattern {
        let column_name = pattern.as_column_name();
        
        if !column_name.is_empty() {
            if let Some(pattern_min) = filters.pattern_min {
                query_builder = query_builder.bind(from_f64(pattern_min));
            }
            if let Some(pattern_max) = filters.pattern_max {
                query_builder = query_builder.bind(from_f64(pattern_max));
            }

            let pattern_search = format!("%\"{}\"%", column_name);
            query_builder = query_builder.bind(pattern_search);
        }
    }

    // Bind BPM filters
    if let Some(bpm_min) = filters.bpm_min {
        query_builder = query_builder.bind(from_f64(bpm_min));
    }

    if let Some(bpm_max) = filters.bpm_max {
        query_builder = query_builder.bind(from_f64(bpm_max));
    }

    // Bind time filters
    if let Some(total_time_min) = filters.total_time_min {
        query_builder = query_builder.bind(total_time_min as i32);
    }

    if let Some(total_time_max) = filters.total_time_max {
        query_builder = query_builder.bind(total_time_max as i32);
    }

    query_builder
}

/// Bind les paramètres de pagination pour les requêtes SELECT
pub fn bind_pagination_params<'q>(
    mut query_builder: sqlx::query::Query<'q, sqlx::Postgres, PgArguments>, 
    filters: &Filters
) -> sqlx::query::Query<'q, sqlx::Postgres, PgArguments> {
    let per_page = filters.per_page.unwrap_or(10);
    let page = filters.page.unwrap_or(1);
    let offset = (page - 1) * per_page;
    
    query_builder = query_builder.bind(per_page as i64);
    query_builder = query_builder.bind(offset as i64);

    query_builder
}

/// Transformer les lignes SQL en structures Rust
pub fn map_rows_to_beatmapsets(rows: Vec<sqlx::postgres::PgRow>) -> Vec<BeatmapsetCompleteShort> {
    let mut map: std::collections::HashMap<i32, BeatmapsetCompleteShort> = HashMap::new();

    for row in rows {
        let beatmapset_id: Option<i32> = row.try_get("beatmapset_id").ok();
        if let Some(bs_id) = beatmapset_id {
            let entry = map.entry(bs_id).or_insert_with(|| BeatmapsetCompleteShort {
                beatmapset: Some(BeatmapsetShort {
                    id: Some(bs_id),
                    osu_id: row.try_get("beatmapset_osu_id").unwrap_or_default(),
                    artist: row.try_get("artist").unwrap_or_default(),
                    title: row.try_get("title").unwrap_or_default(),
                    creator: row.try_get("creator").unwrap_or_default(),
                    cover_url: row.try_get("cover_url").unwrap_or_default(),
                }),
                beatmap: Vec::new(),
            });

            if let Ok(beatmap_id) = row.try_get::<Option<i32>, _>("beatmap_id") {
                if let Some(b_id) = beatmap_id {
                    entry.beatmap.push(BeatmapCompleteShort {
                        beatmap: Some(BeatmapShort {
                            id: Some(b_id),
                            osu_id: row.try_get("beatmap_osu_id").unwrap_or_default(),
                            difficulty: row.try_get("difficulty").unwrap_or_default(),
                            difficulty_rating: row.try_get("difficulty_rating").unwrap_or_default(),
                            mode: row.try_get("mode").unwrap_or_default(),
                            status: row.try_get("status").unwrap_or_default(),
                        }),
                        msd: row.try_get::<Option<i32>, _>("msd_id").ok().flatten().map(|msd_id| MSDShort {
                            id: Some(msd_id),
                            overall: row.try_get("overall").unwrap_or_default(),
                            main_pattern: row.try_get("main_pattern").unwrap_or_default(),
                        }),
                    });
                }
            }
        }
    }

    map.into_iter().map(|(_, v)| v).collect()
}
