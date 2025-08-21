use minacalc_rs::Ssr;

pub fn calculate_main_pattern(ssr: &Ssr) -> String {
    let patterns = vec![
        ("stream", ssr.stream),
        ("jumpstream", ssr.jumpstream),
        ("handstream", ssr.handstream),
        ("stamina", ssr.stamina),
        ("jackspeed", ssr.jackspeed),
        ("chordjack", ssr.chordjack),
        ("technical", ssr.technical),
    ];

    // Trier par rating décroissant
    let mut sorted_patterns: Vec<_> = patterns.into_iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Prendre les 2 premiers patterns
    let top_2: Vec<_> = sorted_patterns
        .into_iter()
        .take(2)
        .map(|(pattern, _)| pattern)
        .collect();

    serde_json::to_string(&top_2).unwrap()
}
