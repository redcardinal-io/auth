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

fn default_port() -> u16 {
    5432
}

fn default_pool_size() -> u32 {
    10
}

fn default_ssl_mode() -> String {
    "prefer".to_string()
}

fn default_migrations_dir() -> String {
    "./migrations".to_string()
}

impl Config {
    pub fn new() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Env::prefixed("RCAUTH_POSTGRES_"))
            .extract()
    }

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
