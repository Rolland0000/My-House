use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

const MAX_CONNECTIONS: u32 = 10;

/// Creates a PostgreSQL connection pool from the provided DSN.
///
/// Runs pending migrations from `migrations/` before returning the pool,
/// ensuring the schema is always up-to-date at startup.
pub async fn connect_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
