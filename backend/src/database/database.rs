use sqlx::{Error, PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {

    /// Creates a new database connection pool with the specified URL
    pub async fn new_connection(url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

        Ok(Self { pool })
    }
    /// Runs database migrations from the ./migrations directory
    pub async fn migrate(&self) -> Result<(), Error> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }
    /// Performs a health check by executing a simple query to verify database connectivity
    pub async fn health_check(&self) -> Result<(), Error> {
        sqlx::query("SELECT 1").fetch_one(&self.pool).await?;

        Ok(())
    }
}
