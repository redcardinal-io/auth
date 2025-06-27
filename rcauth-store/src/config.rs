use serde::Deserialize;

/// Returns the default port for the database (5432).
fn default_port() -> u16 {
    5432
}

/// Returns the default connection pool size (10).
fn default_pool_size() -> u32 {
    10
}

/// Returns the default SSL mode ("prefer").
fn default_ssl_mode() -> String {
    "prefer".to_string()
}

/// Returns the default path for database migrations.
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

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "RCAUTH_POSTGRES_HOST")]
    host: String,
    #[serde(rename = "RCAUTH_POSTGRES_PORT")]
    port: u16,
    #[serde(rename = "RCAUTH_POSTGRES_USER")]
    user: String,
    #[serde(rename = "RCAUTH_POSTGRES_PASSWORD")]
    password: String,
    #[serde(rename = "RCAUTH_POSTGRES_DATABASE")]
    database: String,
    #[serde(rename = "RCAUTH_POSTGRES_POOL_SIZE")]
    pool_size: u32,
    #[serde(rename = "RCAUTH_POSTGRES_SSL_MODE")]
    ssl_mode: String,
    #[serde(rename = "RCAUTH_POSTGRES_MIGRATIONS_DIR")]
    migrations_dir: String,
}

/// Builder for Config to allow for more flexible configuration
#[derive(Default)]
pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    pool_size: Option<u32>,
    ssl_mode: Option<String>,
    migrations_dir: Option<String>,
}

impl Default for Config {
    /// Creates a `Config` instance with default values for all fields.
    ///
    /// Host, user, password, and database are set to empty strings. Port, pool size, SSL mode, and migrations directory use their respective defaults.
    fn default() -> Self {
        Self {
            host: String::new(),
            port: default_port(),
            user: String::new(),
            password: String::new(),
            database: String::new(),
            pool_size: default_pool_size(),
            ssl_mode: default_ssl_mode(),
            migrations_dir: default_migrations_dir(),
        }
    }
}

impl Config {
    /// Returns a new builder for constructing a `Config` instance.
    ///
    /// The builder allows incremental configuration of database connection parameters before validation and creation of a `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::builder()
    ///     .host("localhost")
    ///     .user("postgres")
    ///     .database("mydb")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Constructs a PostgreSQL connection string from the configuration fields.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config {
    ///     host: "localhost".to_string(),
    ///     port: 5432,
    ///     user: "user".to_string(),
    ///     password: "pass".to_string(),
    ///     database: "mydb".to_string(),
    ///     pool_size: 10,
    ///     ssl_mode: "prefer".to_string(),
    ///     migrations_dir: "./migrations".to_string(),
    /// };
    /// let conn_str = config.connection_string();
    /// assert_eq!(
    ///     conn_str,
    ///     "postgres://user:pass@localhost:5432/mydb?sslmode=prefer"
    /// );
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

    /// Returns the path to the migrations directory.
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

    /// Validates the configuration fields for correctness.
    ///
    /// Checks that required fields (host, user, database) are non-empty, the pool size is at least 1, the SSL mode is valid, and the migrations directory exists and is a directory if specified.
    ///
    /// # Errors
    ///
    /// Returns an error if any validation check fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// assert!(config.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check required fields
        if self.host.is_empty() {
            return Err("Database host cannot be empty".into());
        }

        if self.user.is_empty() {
            return Err("Database user cannot be empty".into());
        }

        if self.database.is_empty() {
            return Err("Database name cannot be empty".into());
        }

        // Validate pool size (should be at least 1)
        if self.pool_size == 0 {
            return Err("Pool size must be at least 1".into());
        }

        // Validate SSL mode
        let valid_ssl_modes = ["disable", "prefer", "require", "verify-ca", "verify-full"];
        if !valid_ssl_modes.contains(&self.ssl_mode.as_str()) {
            return Err(format!(
                "Invalid SSL mode: {}. Must be one of: {}",
                self.ssl_mode,
                valid_ssl_modes.join(", ")
            )
            .into());
        }

        // Validate migrations directory exists if specified
        if !self.migrations_dir.is_empty() {
            let path = std::path::Path::new(&self.migrations_dir);
            if !path.exists() || !path.is_dir() {
                return Err(format!(
                    "Migrations directory does not exist or is not a directory: {}",
                    self.migrations_dir
                )
                .into());
            }
        }

        Ok(())
    }
}

impl ConfigBuilder {
    /// Sets the database host for the configuration builder.
    ///
    /// Returns a new builder instance with the specified host set.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().host("localhost");
    /// ```
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Sets the database port for the configuration builder.
    ///
    /// Returns a new builder instance with the specified port set, allowing method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().port(5432);
    /// let config = builder.build().unwrap();
    /// assert_eq!(config.port, 5432);
    /// ```
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Sets the database user for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().user("dbuser");
    /// ```
    pub fn user<S: Into<String>>(mut self, user: S) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Sets the database password for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().password("secret");
    /// ```
    pub fn password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Sets the database name for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().database("mydb");
    /// ```
    pub fn database<S: Into<String>>(mut self, database: S) -> Self {
        self.database = Some(database.into());
        self
    }

    /// Sets the connection pool size for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().pool_size(20);
    /// let config = builder.build().unwrap();
    /// assert_eq!(config.pool_size(), 20);
    /// ```
    pub fn pool_size(mut self, pool_size: u32) -> Self {
        self.pool_size = Some(pool_size);
        self
    }

    /// Sets the SSL mode for the database connection in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().ssl_mode("require");
    /// ```
    pub fn ssl_mode<S: Into<String>>(mut self, ssl_mode: S) -> Self {
        self.ssl_mode = Some(ssl_mode.into());
        self
    }

    /// Sets the migrations directory for the configuration builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().migrations_dir("./db/migrations");
    /// ```
    pub fn migrations_dir<S: Into<String>>(mut self, migrations_dir: S) -> Self {
        self.migrations_dir = Some(migrations_dir.into());
        self
    }

    /// Builds a `Config` instance from the builder, applying defaults for unset fields and validating the result.
    ///
    /// Returns the constructed `Config` if all required fields are valid; otherwise, returns a validation error.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::builder()
    ///     .host("localhost".to_string())
    ///     .user("postgres".to_string())
    ///     .database("mydb".to_string())
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(config.host, "localhost");
    /// ```
    pub fn build(self) -> Result<Config, Box<dyn std::error::Error>> {
        let config = Config {
            host: self.host.unwrap_or_else(String::new),
            port: self.port.unwrap_or_else(default_port),
            user: self.user.unwrap_or_else(String::new),
            password: self.password.unwrap_or_else(String::new),
            database: self.database.unwrap_or_else(String::new),
            pool_size: self.pool_size.unwrap_or_else(default_pool_size),
            ssl_mode: self.ssl_mode.unwrap_or_else(default_ssl_mode),
            migrations_dir: self.migrations_dir.unwrap_or_else(default_migrations_dir),
        };

        // Validate the configuration
        config.validate()?;

        Ok(config)
    }
}
