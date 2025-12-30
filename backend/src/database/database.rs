use sqlx::{Error, PgPool, postgres::PgPoolOptions};

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {

    /// Creates a new database connection pool with the specified URL
    #[tracing::instrument(name = "Creating database connection", skip(url))]
    pub async fn new_connection(url: &str) -> Result<Self, Error> {
        tracing::info!("Initializing database connection pool");
        tracing::debug!("Max connections: 5");
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .map_err(|e| {
                tracing::error!("Failed to connect to database: {:?}", e);
                e
            })?;

        tracing::info!("Database connection pool created successfully");
        Ok(Self { pool })
    }
    
    /// Runs database migrations from the ./migrations directory
    #[tracing::instrument(name = "Running database migrations", skip(self))]
    pub async fn migrate(&self) -> Result<(), Error> {
        tracing::info!("Starting database migrations");
        
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Migration failed: {:?}", e);
                e
            })?;

        tracing::info!("Database migrations completed successfully");
        Ok(())
    }
    
    /// Performs a health check by executing a simple query to verify database connectivity
    #[tracing::instrument(name = "Database health check", skip(self))]
    pub async fn health_check(&self) -> Result<(), Error> {
        tracing::debug!("Executing database health check query");
        
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database health check failed: {:?}", e);
                e
            })?;

        tracing::debug!("Database health check passed");
        Ok(())
    }
}
