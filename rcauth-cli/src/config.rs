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

pub fn load_store_config() -> Result<StoreConfig, figment::Error> {
    Figment::new()
        .merge(Toml::file(&*CONFIG_FILE_PATH))
        .merge(Env::prefixed("RCAUTH_POSTGRES_"))
        .extract()
}

pub fn load_server_config() -> Result<ServerConfig, figment::Error> {
    Figment::new()
        .merge(Toml::file(&*CONFIG_FILE_PATH))
        .merge(Env::prefixed("RCAUTH_SERVER_"))
        .extract()
}

pub fn load_logger_config() -> Result<LoggerConfig, figment::Error> {
    Figment::new()
        .merge(Toml::file(&*CONFIG_FILE_PATH))
        .merge(Env::prefixed("RCAUTH_LOGGER_"))
        .extract()
}
