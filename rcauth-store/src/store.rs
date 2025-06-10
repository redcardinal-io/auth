use async_trait::async_trait;
use rcauth_core::{error::AppError, store::Store};
use snafu::ResultExt;

use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::config::Config;
use crate::error::ConnectionSnafu;

pub struct PgStore {
    pool: sqlx::PgPool,
}

pub async fn new(config: Config) -> Result<PgStore, AppError> {
    info!(
        "🔌 Connecting to PostgreSQL database at {}",
        config.connection_string()
    );
    let pool = PgStore::connect(&config).await?;
    info!("✅ Successfully connected to PostgreSQL database");
    Ok(PgStore { pool })
}

#[async_trait]
impl Store for PgStore {
    type Configuration = Config;
    type Pool = sqlx::PgPool;

    async fn connect(config: &Config) -> Result<sqlx::PgPool, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.pool_size)
            .connect(&config.connection_string())
            .await
            .context(ConnectionSnafu)?;

        Ok(pool)
    }

    async fn run_migrations(&self) -> Result<(), AppError> {
        todo!()
    }

    async fn pool(&self) -> Result<sqlx::PgPool, AppError> {
        todo!()
    }
}
