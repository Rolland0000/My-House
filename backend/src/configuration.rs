use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub db_username: String,
    pub db_password: String,
    pub db_port: u16,
    pub db_host: String,
    pub db_name: String,
}
impl DatabaseSettings {
    /// Constructs a PostgreSQL database connection URL.
    /// 
    /// This method formats the database connection parameters into a standard
    /// PostgreSQL connection string that can be used by database clients.
    /// 
    /// # Returns
    /// 
    /// A `String` containing the formatted PostgreSQL connection URL in the format:
    /// `postgres://username:password@host:port/database_name`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use backend::configuration::DatabaseSettings;
    /// let db_settings = DatabaseSettings {
    ///     db_username: "user".to_string(),
    ///     db_password: "pass".to_string(),
    ///     db_host: "localhost".to_string(),
    ///     db_port: 5432,
    ///     db_name: "mydb".to_string(),
    /// };
    /// 
    /// let url = db_settings.get_database_url();
    /// assert_eq!(url, "postgres://user:pass@localhost:5432/mydb");
    /// ```
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_username, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16,
}

impl Settings {
    /// Loads application settings from configuration files and environment variables.
    /// 
    /// This method attempts to load configuration from:
    /// 1. A configuration file located at "config/configuration"
    /// 2. Environment variables with the prefix "DATABASE_" (using underscore as separator)
    /// 
    /// Environment variables take precedence over file-based configuration.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Settings)` - Successfully loaded and parsed configuration
    /// * `Err(ConfigError)` - Failed to load or parse configuration
    /// 
    /// # Examples
    /// 
    /// ```
    /// use backend::Settings;
    /// let settings = Settings::get_configs().unwrap();
    /// println!("App will run on port: {}", settings.app_port);
    /// 
    /// ```
    pub fn get_configs() -> Result<Settings, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("config/configuration"))
            .add_source(Environment::with_prefix("DATABASE").separator("_"))
            .build()?
            .try_deserialize::<Settings>()?;

        Ok(settings)
    }
}
