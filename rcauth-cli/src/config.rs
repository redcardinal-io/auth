use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use once_cell::sync::Lazy;
use rcauth_core::logger::Config as LoggerConfig;
use rcauth_server::Config as ServerConfig;
use rcauth_store::config::Config as StoreConfig;
use std::env;

pub static CONFIG_FILE_PATH: Lazy<String> =
    Lazy::new(|| env::var("RCAUTH_CONFIG_FILE_PATH").unwrap_or_else(|_| "rcauth.toml".to_string()));

/// Loads the store configuration by merging settings from a TOML file and environment variables.
///
/// The configuration is read from the file specified by `CONFIG_FILE_PATH` and environment variables prefixed with `RCAUTH_POSTGRES_`. Returns the resulting `StoreConfig` or a `figment::Error` if extraction fails.
///
/// # Returns
///
/// A `Result` containing the loaded `StoreConfig` or a `figment::Error` on failure.
///
/// # Examples
///
/// ```
/// let config = load_store_config().expect("Failed to load store config");
/// assert_eq!(config.database_url, "postgres://user:pass@localhost/db");
/// ```
pub fn load_store_config() -> Result<StoreConfig, figment::Error> {
    Figment::new()
        .merge(Toml::file(&*CONFIG_FILE_PATH))
        .merge(Env::prefixed("RCAUTH_POSTGRES_"))
        .extract()
}

/// Loads the server configuration by merging values from a TOML file and environment variables.
///
/// The configuration is read from the file specified by `CONFIG_FILE_PATH` and environment variables prefixed with `RCAUTH_SERVER_`. Returns the resulting `ServerConfig` or a `figment::Error` if extraction fails.
///
/// # Examples
///
/// ```
/// let config = load_server_config().expect("Failed to load server config");
/// ```
pub fn load_server_config() -> Result<ServerConfig, figment::Error> {
    Figment::new()
        .merge(Toml::file(&*CONFIG_FILE_PATH))
        .merge(Env::prefixed("RCAUTH_SERVER_"))
        .extract()
}

/// Loads the logger configuration by merging settings from a TOML file and environment variables.
///
/// The configuration is read from the file specified by `CONFIG_FILE_PATH` and environment variables prefixed with `RCAUTH_LOGGER_`. Returns the resulting `LoggerConfig` or a `figment::Error` if extraction fails.
///
/// # Examples
///
/// ```
/// let config = load_logger_config().unwrap();
/// assert_eq!(config.level, "info");
/// ```
pub fn load_logger_config() -> Result<LoggerConfig, figment::Error> {
    Figment::new()
        .merge(Toml::file(&*CONFIG_FILE_PATH))
        .merge(Env::prefixed("RCAUTH_LOGGER_"))
        .extract()
}
