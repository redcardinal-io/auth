mod migrate;
mod serve;

use clap::{Parser, Subcommand};
use dotenvy;
use rcauth_core::error::Result;
use rcauth_store::config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    // print cwd
    println!(
        "Current working directory: {}",
        std::env::current_dir().unwrap().display()
    );

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

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
