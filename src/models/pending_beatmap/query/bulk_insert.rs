use sqlx::{Error as SqlxError, PgPool};

pub async fn bulk_insert(pool: &PgPool, hashes: &[String]) -> Result<usize, SqlxError> {
    if hashes.is_empty() {
        return Ok(0);
    }

    let placeholders: Vec<String> = (1..=hashes.len()).map(|i| format!("(${})", i)).collect();

    let query = format!(
        "INSERT INTO pending_beatmap (hash) VALUES {} ON CONFLICT (hash) DO NOTHING",
        placeholders.join(", ")
    );

    let mut q = sqlx::query(&query);
    for hash in hashes {
        q = q.bind(hash);
    }

    // Retourne le nombre de lignes affectées
    let result = q.execute(pool).await?;
    Ok(result.rows_affected() as usize)
}
