use crate::models::short::complete::types::{BeatmapCompleteShort, BeatmapsetCompleteShort};
use crate::models::short::msd::MSDShort;
use crate::models::short::beatmap::BeatmapShort;
use crate::models::short::beatmapset::BeatmapsetShort;
use crate::models::Filters;
use std::collections::HashMap;
use sqlx::{Error as SqlxError, PgPool, Row};
use crate::helpers::common::from_f64;

pub async fn find_by_filters(
    pool: &PgPool,
    filters: &Filters,
) -> Result<Vec<BeatmapsetCompleteShort>, SqlxError> {

    let (query, param_count) = build_query(filters);
    let query_builder = bind_params(sqlx::query(&query), filters);
    let rows = query_builder.fetch_all(pool).await?;

    Ok(map_rows_to_beatmapsets(rows))
}

// -------------------------------------------
// Construire la requête SQL avec les filtres
// -------------------------------------------
fn build_query(filters: &Filters) -> (String, usize) {
    let mut query = String::from(
        r#"
        SELECT 
            bs.id as beatmapset_id, bs.osu_id as beatmapset_osu_id, bs.artist, bs.title, bs.creator, bs.cover_url,
            b.id as beatmap_id, b.osu_id as beatmap_osu_id, b.difficulty, b.difficulty_rating, b.mode, b.status,
            m.id as msd_id, m.overall, m.main_pattern
        FROM beatmapset bs
        LEFT JOIN beatmap b ON bs.id = b.beatmapset_id
        LEFT JOIN msd m ON b.id = m.beatmap_id
        "#
    );

    let mut conditions: Vec<String> = Vec::new();
    let mut param_count = 0;

    if let Some(search_term) = &filters.search_term {
        if !search_term.is_empty() {
            param_count += 1;
            conditions.push(format!(
                "(b.difficulty ILIKE ${} OR b.status ILIKE ${} OR bs.artist ILIKE ${} OR bs.title ILIKE ${} OR bs.creator ILIKE ${})",
                param_count, param_count, param_count, param_count, param_count
            ));
        }
    }

    // Vérifier que m.overall existe avant d'appliquer les filtres
    if filters.overall_min.is_some() || filters.overall_max.is_some() {
        conditions.push("m.overall IS NOT NULL".to_string());
    }

    if let Some(overall_min) = filters.overall_min {
        param_count += 1;
        conditions.push(format!("m.overall >= ${}", param_count));
    }

    if let Some(overall_max) = filters.overall_max {
        param_count += 1;
        conditions.push(format!("m.overall <= ${}", param_count));
    }

    // Vérifier que le pattern sélectionné est valide et que m existe
    if let Some(pattern) = &filters.selected_pattern {
        if !pattern.is_empty() {
            // Vérifier que le pattern est une colonne valide
            let valid_patterns = ["stream", "jumpstream", "handstream", "stamina", "jackspeed", "chordjack", "technical"];
            if valid_patterns.contains(&pattern.as_str()) {
                conditions.push("m.id IS NOT NULL".to_string());
                
                if let Some(pattern_min) = filters.pattern_min {
                    param_count += 1;
                    conditions.push(format!("m.{} >= ${}", pattern, param_count));
                }
                if let Some(pattern_max) = filters.pattern_max {
                    param_count += 1;
                    conditions.push(format!("m.{} <= ${}", pattern, param_count));
                }

                param_count += 1;
                conditions.push(format!("m.main_pattern ILIKE ${}", param_count));
            }
        }
    }

    // Ajouter la condition rate = 1.0 seulement si on filtre par MSD
    if filters.overall_min.is_some() || filters.overall_max.is_some() || 
       (filters.selected_pattern.is_some() && !filters.selected_pattern.as_ref().unwrap().is_empty()) {
        conditions.push("m.rate = 1.0".to_string());
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    
    query.push_str(" ORDER BY bs.id, b.id");

    // Ajouter la pagination
    let per_page = filters.per_page.unwrap_or(10);
    let page = filters.page.unwrap_or(1);
    let offset = (page - 1) * per_page;
    
    param_count += 1;
    query.push_str(&format!(" LIMIT ${}", param_count));
    param_count += 1;
    query.push_str(&format!(" OFFSET ${}", param_count));

    (query, param_count)
}

// -------------------------------------------
// Lier les paramètres à la requête SQL
// -------------------------------------------
fn bind_params<'q>(mut query_builder: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>, filters: &Filters)
    -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>
{
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

    // Lier les paramètres de pagination
    let per_page = filters.per_page.unwrap_or(10);
    let page = filters.page.unwrap_or(1);
    let offset = (page - 1) * per_page;
    
    query_builder = query_builder.bind(per_page as i64);
    query_builder = query_builder.bind(offset as i64);

    query_builder
}

// -------------------------------------------
// Transformer les lignes SQL en structures Rust
// -------------------------------------------
fn map_rows_to_beatmapsets(rows: Vec<sqlx::postgres::PgRow>) -> Vec<BeatmapsetCompleteShort> {
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
