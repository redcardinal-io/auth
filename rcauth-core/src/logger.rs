use crate::error::{Error, ErrorCode, Result};
use serde::Deserialize;
use tracing::Level;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    #[serde(rename = "RCAUTH_LOG_LEVEL", default = "default_log_level")]
    pub log_level: LogLevel,
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
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

impl LogConfig {
    pub fn init(&self) {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::from(self.log_level.clone()))
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    }

    pub fn from_env() -> Result<Self> {
        envy::keep_names()
            .from_env()
            .map_err(|e| Error::new(ErrorCode::Internal, "Failed to load log configuration", e))
    }
}
