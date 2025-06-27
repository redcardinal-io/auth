use crate::error::{Error, ErrorCode};
use serde::Deserialize;
use tracing::Level;

/// Returns the default log level (Info).
fn default_log_level() -> String {
    "info".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "RCAUTH_LOG_LEVEL")]
    pub log_level: String,
}

#[derive(Default)]
pub struct ConfigBuilder {
    log_level: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
        }
    }
}

impl Config {
    /// Creates a new builder for constructing a Config
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Get the tracing Level from the config
    pub fn level(&self) -> Level {
        match self.log_level.to_lowercase().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO, // Default to INFO for invalid levels
        }
    }

    /// Initialize the global logger with the config
    pub fn init(&self) {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(self.level())
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate log level
        let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
        let log_level = self.log_level.to_lowercase();

        if !valid_log_levels.contains(&log_level.as_str()) {
            return Err(format!(
                "Invalid log level: {}. Must be one of: {}",
                self.log_level,
                valid_log_levels.join(", ")
            )
            .into());
        }

        Ok(())
    }
}

impl ConfigBuilder {
    /// Set the log level
    pub fn log_level<S: Into<String>>(mut self, log_level: S) -> Self {
        self.log_level = Some(log_level.into());
        self
    }

    /// Build the Config object
    pub fn build(self) -> Result<Config, Box<dyn std::error::Error>> {
        let config = Config {
            log_level: self.log_level.unwrap_or_else(default_log_level),
        };

        // Validate the configuration
        config.validate()?;

        Ok(config)
    }
}

fn load_from_env() -> Result<Config, Box<dyn std::error::Error>> {
    let mut builder = Config::builder();

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

// For backward compatibility
pub type LogConfig = Config;
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}
