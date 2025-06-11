mod migrate;
mod serve;

use clap::{Parser, Subcommand};
use dotenvy;
use rcauth_core::{error::Result, logger::LogConfig};
use rcauth_store::config::Config;

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
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    LogConfig::from_env()?.init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Migrate => {
            let config = Config::from_env()?;
            migrate::run(config).await?;
        }
        Commands::Serve => {
            serve::run().await?;
        }
    }

    Ok(())
}
