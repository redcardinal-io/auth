use crate::error::{Error, ErrorCode};
use serde::Deserialize;
use tracing::Level;

/// Returns the default log level as a string ("info").
///
/// # Examples
///
/// ```
/// let level = default_log_level();
/// assert_eq!(level, "info");
/// ```
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
    /// Creates a `Config` instance with the default log level set to "info".
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// assert_eq!(config.log_level, "info");
    /// ```
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
        }
    }
}

impl Config {
    /// Returns a new builder for constructing a `Config` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = Config::builder();
    /// let config = builder.log_level("debug").build().unwrap();
    /// assert_eq!(config.log_level, "debug");
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Returns the corresponding `tracing::Level` for the configured log level string.
    ///
    /// If the log level string is unrecognized, defaults to `Level::INFO`.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config { log_level: "debug".to_string() };
    /// assert_eq!(config.level(), tracing::Level::DEBUG);
    /// ```
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

    /// Initializes the global tracing subscriber with the configured log level and target logging enabled.
    ///
    /// Panics if setting the global subscriber fails.
    pub fn init(&self) {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(self.level())
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    }

    /// Checks if the configured log level is valid.
    ///
    /// Returns an error if `log_level` is not one of "trace", "debug", "info", "warn", or "error".
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
    /// Sets the log level for the configuration builder.
    ///
    /// Returns a new builder instance with the specified log level set.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ConfigBuilder::default().log_level("debug");
    /// let config = builder.build().unwrap();
    /// assert_eq!(config.log_level, "debug");
    /// ```
    pub fn log_level<S: Into<String>>(mut self, log_level: S) -> Self {
        self.log_level = Some(log_level.into());
        self
    }

    /// Constructs a `Config` instance from the builder, validating the log level.
    ///
    /// Returns an error if the log level is invalid.
    ///
    /// # Returns
    ///
    /// - `Ok(Config)` if the configuration is valid.
    /// - `Err` if the log level is not one of the accepted values.
    pub fn build(self) -> Result<Config, Box<dyn std::error::Error>> {
        let config = Config {
            log_level: self.log_level.unwrap_or_else(default_log_level),
        };

        // Validate the configuration
        config.validate()?;

        Ok(config)
    }
}

/// Loads the logging configuration from the `RCAUTH_LOG_LEVEL` environment variable.
///
/// Returns a `Config` instance with the log level set from the environment variable if present, or the default if not.
/// If the log level is invalid, returns a configuration error wrapped in a custom error type.
///
/// # Returns
///
/// - `Ok(Config)` if the configuration is valid.
/// - `Err(Box<dyn std::error::Error>)` if the log level is invalid or another error occurs.
///
/// # Examples
///
/// ```
/// std::env::set_var("RCAUTH_LOG_LEVEL", "debug");
/// let config = load_from_env().unwrap();
/// assert_eq!(config.log_level, "debug");
/// ```
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
    /// Converts a `LogLevel` variant to the corresponding `tracing::Level`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcauth_core::logger::{LogLevel};
    /// use tracing::Level;
    ///
    /// let level = Level::from(LogLevel::Warn);
    /// assert_eq!(level, Level::WARN);
    /// ```
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
