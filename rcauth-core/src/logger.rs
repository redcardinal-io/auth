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

fn default_log_level() -> String {
    "info".to_string()
}

impl Config {
    pub fn new() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Env::prefixed("RCAUTH_LOGGER_"))
            .extract()
    }

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

    pub fn init(&self) {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(self.level())
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
        }
    }
}
