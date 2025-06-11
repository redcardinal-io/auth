use crate::error::ConnectionSnafu;
use crate::{config::Config, error::MigrationSnafu};
use async_trait::async_trait;
use rcauth_core::{error::Result, store::Store};
use snafu::ResultExt;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use tracing::{debug, info};

pub struct PgStore {
    pool: sqlx::PgPool,
    migrations_dir: String,
}

pub async fn new(config: Config) -> Result<PgStore> {
    info!("🔌 Connecting to PostgreSQL database");
    match PgStore::connect(&config).await {
        Ok(pool) => {
            info!("✅ Successfully connected to PostgreSQL database");
            Ok(PgStore {
                pool,
                migrations_dir: config.migrations_dir().to_string(),
            })
        }
        Err(err) => {
            tracing::error!("❌ Failed to connect to PostgreSQL database: {}", err);
            Err(err)
        }
    }
}

#[async_trait]
impl Store for PgStore {
    type Configuration = Config;
    type Pool = sqlx::PgPool;

    async fn connect(config: &Config) -> Result<sqlx::PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(config.pool_size())
            .connect(&config.connection_string())
            .await
            .context(ConnectionSnafu)?;

        Ok(pool)
    }

    async fn run_migrations(&self) -> Result<()> {
        let migrations_dir = Path::new(self.migrations_dir.as_str());

        debug!("Running migrations from directory: {:?}", migrations_dir);

        let _ = sqlx::migrate::Migrator::new(migrations_dir)
            .await
            .context(MigrationSnafu)?
            .run(&self.pool)
            .await
            .context(MigrationSnafu)?;

        Ok(())
    }

    async fn pool(&self) -> Result<sqlx::PgPool> {
        Ok(self.pool.clone())
    }
}
