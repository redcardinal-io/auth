use figment::{
    providers::Env,
    Figment,
};
use serde::Deserialize;
use tracing::Level;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

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

impl Config {
    /// Loads logger configuration from environment variables.
    ///
    /// Reads configuration values from environment variables prefixed with `RCAUTH_LOGGER_` and constructs a `Config` instance. Returns an error if extraction fails.
    ///
    /// # Returns
    ///
    /// A `Result` containing the loaded `Config` or a `figment::Error` if configuration extraction fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::new().expect("Failed to load logger config");
    /// ```
    pub fn new() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Env::prefixed("RCAUTH_LOGGER_"))
            .extract()
    }

    /// Returns the corresponding `tracing::Level` for the configured log level string.
    ///
    /// Converts the `log_level` field to a `tracing::Level` variant. If the value is unrecognized, defaults to `Level::INFO`.
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
            _ => Level::INFO,
        }
    }

    /// Initializes the global tracing subscriber with the configured log level.
    ///
    /// Sets up a formatted tracing subscriber using the log level specified in this configuration.
    /// Panics if the global subscriber cannot be set.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::default();
    /// config.init();
    /// // Logging is now initialized at the default "info" level.
    /// ```
    pub fn init(&self) {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(self.level())
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    }
}

impl Default for Config {
    /// Returns a `Config` instance with the default log level set to `"info"`.
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
