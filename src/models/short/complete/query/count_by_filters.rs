use crate::models::Filters;
use sqlx::{Error as SqlxError, PgPool, Row};
use crate::helpers::common::from_f64;

pub async fn count_by_filters(
    pool: &PgPool,
    filters: &Filters,
) -> Result<i64, SqlxError> {

    // Construire la requête COUNT
    let mut query = String::from(
        r#"
        SELECT COUNT(DISTINCT bs.id) as total
        FROM beatmapset bs
        LEFT JOIN beatmap b ON bs.id = b.beatmapset_id
        LEFT JOIN msd m ON b.id = m.beatmap_id
        "#
    );

    let mut conditions: Vec<String> = Vec::new();
    let mut param_count = 0;

    // Réutiliser la logique de filtrage
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

    let mut query_builder = sqlx::query(&query);

    // Lier les paramètres
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
            let valid_patterns = ["stream", "jumpstream", "handstream", "stamina", "jackspeed", "chordjack", "technical"];
            if valid_patterns.contains(&pattern.as_str()) {
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
    }

    // Exécuter la requête et récupérer le total
    let row = query_builder.fetch_one(pool).await?;
    let total: i64 = row.try_get("total")?;
    Ok(total)
}
