use rcauth_core::{error::Result, store::Store};
use rcauth_store::config::Config;
use tracing::info;

pub async fn run(config: Config) -> Result<()> {
    info!("Starting database migration");

    // Connect to the database
    let store = rcauth_store::store::new(config).await?;

    // Run migrations
    store.run_migrations().await?;

    info!("✅ Database migration completed successfully");
    Ok(())
}
