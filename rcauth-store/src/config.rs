use rcauth_core::error::Result;
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

/// Returns the default directory for database migrations ("./migrations").
fn default_migrations_dir() -> String {
    "./migrations".to_string()
}

/// Configuration for the database connection.
#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "RCAUTH_DB_HOST")]
    host: String,
    #[serde(rename = "RCAUTH_DB_PORT", default = "default_port")]
    port: u16,
    #[serde(rename = "RCAUTH_DB_USER")]
    user: String,
    #[serde(rename = "RCAUTH_DB_PASSWORD")]
    password: String,
    #[serde(rename = "RCAUTH_DB_NAME")]
    database: String,
    #[serde(rename = "RCAUTH_DB_POOL_SIZE", default = "default_pool_size")]
    pool_size: u32,
    #[serde(rename = "RCAUTH_DB_SSLMODE", default = "default_ssl_mode")]
    ssl_mode: String,
    #[serde(
        rename = "RCAUTH_DB_MIGRATIONS_DIR",
        default = "default_migrations_dir"
    )]
    migrations_dir: String,
}

impl Default for Config {
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
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.user, self.password, self.host, self.port, self.database, self.ssl_mode
        )
    }

    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }

    pub fn migrations_dir(&self) -> &str {
        &self.migrations_dir
    }

    pub fn from_env() -> Result<Self> {
        envy::keep_names().from_env::<Self>().map_err(|e| {
            rcauth_core::error::Error::new(
                rcauth_core::error::ErrorCode::Internal,
                // The error message is now more informative.
                "Failed to load configuration. Check for missing required environment variables like RCAUTH_DB_HOST",
                e,
            )
        })
    }
}
