use sqlx::query;
use sqlx::Row;

/// How long to sit in the listen loop before exiting.
///
/// This ensures the example eventually exits, which is required for automated testing.
// const LISTEN_DURATION: Duration = Duration::from_secs(5);

#[tokio::main]
async fn make() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building PG pool.");
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = sqlx::PgPool::connect(&conn_str).await?;

    let res: Vec<sqlx::postgres::PgRow> = query(
        "INSERT INTO genre (genre_id,name_genre) 
        VALUES (1,'Роман');",
    )
    .fetch_all(&pool)
    .await?;
    Ok(())
}
