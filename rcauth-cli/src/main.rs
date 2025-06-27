mod config;
mod migrate;
mod serve;

use clap::{Parser, Subcommand};
use dotenvy;
use tracing::info;

use crate::config::{load_logger_config, load_store_config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run database migrations
    Migrate,

    /// Start the authentication server
    Serve,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Initialize logging
    load_logger_config()?.init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Load database configuration
    let store_config = load_store_config()?;
    info!("🛢️ Database configuration loaded successfully");

    match &cli.command {
        Commands::Migrate => migrate::run(store_config).await?,
        Commands::Serve => serve::run().await?,
    }

    Ok(())
}
