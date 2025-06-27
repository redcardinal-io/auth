use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,
    #[serde(default = "default_ssl_mode")]
    pub ssl_mode: String,
    #[serde(default = "default_migrations_dir")]
    pub migrations_dir: String,
}

/// Returns the default PostgreSQL port number (5432).
///
/// # Examples
///
/// ```
/// let port = default_port();
/// assert_eq!(port, 5432);
/// ```
fn default_port() -> u16 {
    5432
}

/// Returns the default connection pool size for the database configuration.
///
/// # Examples
///
/// ```
/// let size = default_pool_size();
/// assert_eq!(size, 10);
/// ```
fn default_pool_size() -> u32 {
    10
}

/// Returns the default SSL mode for PostgreSQL connections, which is "prefer".
///
/// # Examples
///
/// ```
/// let ssl_mode = default_ssl_mode();
/// assert_eq!(ssl_mode, "prefer");
/// ```
fn default_ssl_mode() -> String {
    "prefer".to_string()
}

/// Returns the default directory path for database migrations.
///
/// # Examples
///
/// ```
/// let dir = default_migrations_dir();
/// assert_eq!(dir, "./migrations");
/// ```
fn default_migrations_dir() -> String {
    "./migrations".to_string()
}

impl Config {
    /// Loads PostgreSQL configuration from environment variables with the `RCAUTH_POSTGRES_` prefix.
    ///
    /// Returns a `Config` instance populated from the environment, or a `figment::Error` if loading fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::new().expect("Failed to load config");
    /// ```
    pub fn new() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Env::prefixed("RCAUTH_POSTGRES_"))
            .extract()
    }

    /// Constructs a PostgreSQL connection string using the current configuration.
    ///
    /// # Returns
    ///
    /// A connection string in the format:
    /// `postgres://user:password@host:port/database?sslmode=ssl_mode`.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// let conn_str = config.connection_string();
    /// assert!(conn_str.starts_with("postgres://"));
    /// ```
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.user, self.password, self.host, self.port, self.database, self.ssl_mode
        )
    }

    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }

    /// Returns the path to the migrations directory configured for the database.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// let dir = config.migrations_dir();
    /// assert_eq!(dir, "./migrations");
    /// ```
    pub fn migrations_dir(&self) -> &str {
        &self.migrations_dir
    }

    /// Validates that required database configuration fields are not empty.
    ///
    /// Returns an error if any of the `host`, `user`, `password`, or `database` fields are empty; otherwise, returns `Ok(())`.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// assert!(config.validate().is_ok());
    ///
    /// let mut invalid_config = Config::default();
    /// invalid_config.host = "".to_string();
    /// assert!(invalid_config.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.host.is_empty() {
            return Err("Database host cannot be empty".into());
        }
        if self.user.is_empty() {
            return Err("Database user cannot be empty".into());
        }
        if self.password.is_empty() {
            return Err("Database password cannot be empty".into());
        }
        if self.database.is_empty() {
            return Err("Database name cannot be empty".into());
        }
        Ok(())
    }
}

impl Default for Config {
    /// Returns a `Config` instance with default values for all fields.
    ///
    /// The default configuration uses "localhost" for the host, "postgres" for the user,
    /// "password" for the password, "rcauth" for the database, and standard defaults for
    /// port, pool size, SSL mode, and migrations directory.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// assert_eq!(config.host, "localhost");
    /// assert_eq!(config.user, "postgres");
    /// ```
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: default_port(),
            user: "postgres".to_string(),
            password: "password".to_string(),
            database: "rcauth".to_string(),
            pool_size: default_pool_size(),
            ssl_mode: default_ssl_mode(),
            migrations_dir: default_migrations_dir(),
        }
    }
}
