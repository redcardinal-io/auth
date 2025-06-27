use lazy_static::lazy_static;
use rcauth_core::{
    error::{Error, ErrorCode},
    logger::Config as LoggerConfig,
};
use rcauth_server::Config as ServerConfig;
use rcauth_store::config::Config as StoreConfig;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use tracing::{debug, info};

// Define a struct to hold all config types
#[derive(Default)]
struct ConfigFile {
    store_config: Option<StoreConfig>,
    server_config: Option<ServerConfig>,
    logger_config: Option<LoggerConfig>,
}

lazy_static! {
    static ref CONFIG: Arc<ConfigFile> = Arc::new(load_config_file());
}

// Loads the configuration file once and returns the cached config
/// Loads configuration from the `rcauth.toml` file if it exists.
///
/// Attempts to parse the file content into `StoreConfig`, `ServerConfig`, and `LoggerConfig` independently.
/// Returns a `ConfigFile` struct containing any successfully parsed configurations; fields remain `None` if parsing fails or the file is missing.
fn load_config_file() -> ConfigFile {
    const CONFIG_PATH: &str = "rcauth.toml";
    let mut config = ConfigFile::default();

    if Path::new(CONFIG_PATH).exists() {
        debug!("Found config file at {}", CONFIG_PATH);
        match fs::read_to_string(CONFIG_PATH) {
            Ok(content) => {
                if let Ok(parsed_config) = toml::from_str::<StoreConfig>(&content) {
                    config.store_config = Some(parsed_config);
                }

                if let Ok(parsed_config) = toml::from_str::<ServerConfig>(&content) {
                    config.server_config = Some(parsed_config);
                }

                if let Ok(parsed_config) = toml::from_str::<LoggerConfig>(&content) {
                    config.logger_config = Some(parsed_config);
                }
            }
            Err(e) => {
                debug!("Failed to read config file {}: {}", CONFIG_PATH, e);
            }
        }
    } else {
        debug!("Config file not found at {}", CONFIG_PATH);
    }

    config
}

/// Loads the database store configuration from a cached configuration file or environment variables.
///
/// Attempts to retrieve the `StoreConfig` from a cached configuration file (`rcauth.toml`). If not present, falls back to loading from environment variables. Returns an error if configuration cannot be loaded from either source.
///
/// # Returns
///
/// A `StoreConfig` instance on success, or a boxed error if loading fails.
///
/// # Examples
///
/// ```
/// let config = load_store_config().expect("Failed to load store configuration");
/// assert_eq!(config.db_user, "postgres");
/// ```
pub fn load_store_config() -> Result<StoreConfig, Box<dyn std::error::Error>> {
    // Try to load from config file first
    if let Some(config) = &CONFIG.store_config {
        info!("Loaded database configuration from file");
        return Ok(config.clone());
    }

    // Fall back to environment variables
    match load_store_from_env() {
        Ok(config) => {
            info!("Loaded database configuration from environment variables");
            Ok(config)
        }
        Err(err) => {
            debug!("Failed to load config from environment: {:?}", err);
            Err(Box::new(Error::new_simple(
                ErrorCode::ConfigurationError,
                "Failed to load database configuration from environment or config file",
            )) as Box<dyn std::error::Error>)
        }
    }
}

/// Loads the server configuration from a cached configuration file or environment variables.
///
/// Attempts to retrieve the `ServerConfig` from the cached configuration file. If not present,
/// falls back to loading the configuration from environment variables. Returns an error if the
/// configuration cannot be loaded from either source.
///
/// # Returns
///
/// A `Result` containing the loaded `ServerConfig` on success, or a boxed error if loading fails.
///
/// # Examples
///
/// ```
/// let server_config = load_server_config()?;
/// println!("API server will run on {}:{}", server_config.api_host, server_config.api_port);
/// ```
pub fn load_server_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    // Try to load from config file first
    if let Some(config) = &CONFIG.server_config {
        info!("Loaded server configuration from file");
        return Ok(config.clone());
    }

    // Fall back to environment variables
    let config = load_server_from_env()?;
    info!("Loaded server configuration from environment variables");
    Ok(config)
}

/// Loads the logger configuration from the cached configuration file or environment variables.
///
/// Attempts to retrieve the logger configuration from the cached file-based config. If unavailable, falls back to loading from environment variables. Returns an error if neither source provides a valid configuration.
///
/// # Returns
///
/// A `LoggerConfig` instance on success, or an error if loading fails.
///
/// # Examples
///
/// ```
/// let logger_config = load_logger_config().unwrap();
/// assert_eq!(logger_config.level, "info");
/// ```
pub fn load_logger_config() -> Result<LoggerConfig, Box<dyn std::error::Error>> {
    // Try to load from config file first
    if let Some(config) = &CONFIG.logger_config {
        info!("Loaded logger configuration from file");
        return Ok(config.clone());
    }

    // Fall back to environment variables
    let config = load_logger_from_env()?;
    info!("Loaded logger configuration from environment variables");
    Ok(config)
}

/// Loads the store (database) configuration from environment variables.
///
/// Reads environment variables related to database connection settings, parses and validates them,
/// and constructs a `StoreConfig` instance. Returns an error if any required value is invalid or missing.
///
/// # Returns
///
/// - `Ok(StoreConfig)` if all required environment variables are present and valid.
/// - `Err(Box<dyn std::error::Error>)` if any configuration is invalid or missing.
///
/// # Examples
///
/// ```
/// std::env::set_var("RCAUTH_DB_HOST", "localhost");
/// std::env::set_var("RCAUTH_DB_PORT", "5432");
/// std::env::set_var("RCAUTH_DB_USER", "user");
/// std::env::set_var("RCAUTH_DB_PASSWORD", "pass");
/// std::env::set_var("RCAUTH_DB_NAME", "mydb");
/// let config = load_store_from_env().unwrap();
/// assert_eq!(config.host, "localhost");
/// ```
fn load_store_from_env() -> Result<StoreConfig, Box<dyn std::error::Error>> {
    let mut builder = StoreConfig::builder();

    if let Ok(host) = std::env::var("RCAUTH_DB_HOST") {
        builder = builder.host(host);
    }

    if let Ok(port_str) = std::env::var("RCAUTH_DB_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            builder = builder.port(port);
        }
    }

    if let Ok(user) = std::env::var("RCAUTH_DB_USER") {
        builder = builder.user(user);
    }

    if let Ok(password) = std::env::var("RCAUTH_DB_PASSWORD") {
        builder = builder.password(password);
    }

    if let Ok(database) = std::env::var("RCAUTH_DB_NAME") {
        builder = builder.database(database);
    }

    if let Ok(pool_size_str) = std::env::var("RCAUTH_DB_POOL_SIZE") {
        if let Ok(pool_size) = pool_size_str.parse::<u32>() {
            builder = builder.pool_size(pool_size);
        }
    }

    if let Ok(ssl_mode) = std::env::var("RCAUTH_DB_SSL_MODE") {
        builder = builder.ssl_mode(ssl_mode);
    }

    if let Ok(migrations_dir) = std::env::var("RCAUTH_DB_MIGRATIONS_DIR") {
        builder = builder.migrations_dir(migrations_dir);
    }

    builder.build().map_err(|e| {
        Box::new(Error::new_simple(
            ErrorCode::ConfigurationError,
            format!("Invalid database configuration: {}", e),
        )) as Box<dyn std::error::Error>
    })
}

/// Loads the server configuration from environment variables.
///
/// Reads environment variables related to API and management server hosts and ports, Swagger and CORS enable flags, and allowed CORS origins. Constructs and returns a `ServerConfig` if all required values are valid; otherwise, returns a configuration error.
///
/// # Returns
///
/// A `ServerConfig` instance if environment variables are valid; otherwise, an error.
///
/// # Examples
///
/// ```
/// std::env::set_var("RCAUTH_API_SERVER_HOST", "127.0.0.1");
/// std::env::set_var("RCAUTH_API_SERVER_PORT", "8080");
/// let config = load_server_from_env().unwrap();
/// assert_eq!(config.api_server_host, "127.0.0.1");
/// assert_eq!(config.api_server_port, 8080);
/// ```
fn load_server_from_env() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let mut builder = ServerConfig::builder();

    if let Ok(host) = std::env::var("RCAUTH_API_SERVER_HOST") {
        builder = builder.api_server_host(host);
    }

    if let Ok(port_str) = std::env::var("RCAUTH_API_SERVER_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            builder = builder.api_server_port(port);
        }
    }

    if let Ok(host) = std::env::var("RCAUTH_MANAGEMENT_SERVER_HOST") {
        builder = builder.management_server_host(host);
    }

    if let Ok(port_str) = std::env::var("RCAUTH_MANAGEMENT_SERVER_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            builder = builder.management_server_port(port);
        }
    }

    if let Ok(swagger) = std::env::var("RCAUTH_ENABLE_SWAGGER") {
        builder = builder.enable_swagger(swagger == "true");
    }

    if let Ok(cors) = std::env::var("RCAUTH_ENABLE_CORS") {
        builder = builder.enable_cors(cors == "true");
    }

    if let Ok(origins) = std::env::var("RCAUTH_CORS_ALLOWED_ORIGINS") {
        let origins_vec: Vec<String> = origins.split(',').map(String::from).collect();
        builder = builder.cors_allowed_origins(origins_vec);
    }

    builder.build().map_err(|e| {
        Box::new(Error::new_simple(
            ErrorCode::ConfigurationError,
            format!("Invalid server configuration: {}", e),
        )) as Box<dyn std::error::Error>
    })
}

// The load_*_from_file functions have been consolidated into load_config_file

/// Loads the logger configuration from environment variables.
///
/// Reads the `RCAUTH_LOG_LEVEL` environment variable to set the log level, if present. Returns a `LoggerConfig` on success, or a configuration error if the environment variable is invalid or the configuration cannot be built.
///
/// # Returns
/// A `LoggerConfig` instance if the environment variable is valid; otherwise, an error describing the configuration issue.
///
/// # Examples
///
/// ```
/// let logger_config = load_logger_from_env().unwrap();
/// assert!(logger_config.log_level().is_some());
/// ```
fn load_logger_from_env() -> Result<LoggerConfig, Box<dyn std::error::Error>> {
    let mut builder = LoggerConfig::builder();

    if let Ok(log_level) = std::env::var("RCAUTH_LOG_LEVEL") {
        builder = builder.log_level(log_level);
    }

    builder.build().map_err(|e| {
        Box::new(Error::new_simple(
            ErrorCode::ConfigurationError,
            format!("Invalid logger configuration: {}", e),
        )) as Box<dyn std::error::Error>
    })
}
