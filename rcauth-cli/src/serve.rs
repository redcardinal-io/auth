use rcauth_core::error::{Error, ErrorCode};
use tokio::task::JoinSet;
use tracing::{error, info};

use crate::config;

/// Starts and manages the authentication API server and management server concurrently.
///
/// Loads the server configuration, then launches both the API and management servers as asynchronous tasks.
/// The function waits for either server to exit or panic, returning an error if this occurs. Both servers are expected to run indefinitely; reaching the end of this function is considered abnormal.
///
/// # Returns
///
/// Returns `Ok(())` if both servers run indefinitely (unexpected), or an error if a server exits or panics.
///
/// # Examples
///
/// ```no_run
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     rcauth_cli::serve::run().await?;
///     Ok(())
/// }
/// ```
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting authentication server");

    // Load server configuration
    let server_config = config::load_server_config()?;
    info!("🔧 Server configuration loaded successfully");

    // Create a JoinSet to run both servers concurrently
    let mut tasks = JoinSet::new();

    // Clone config and store for each server
    let api_config = server_config.clone();
    let mgmt_config = server_config;

    // Start API server
    tasks.spawn(async move {
        match rcauth_server::run_api_server(&api_config).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("API server error: {}", e);
                Err(Error::new_simple(
                    ErrorCode::ServerError,
                    format!("API server failed: {}", e),
                ))
            }
        }
    });

    // Start management server
    tasks.spawn(async move {
        match rcauth_server::run_management_server(&mgmt_config).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Management server error: {}", e);
                Err(Error::new_simple(
                    ErrorCode::ServerError,
                    format!("Management server failed: {}", e),
                ))
            }
        }
    });

    // Wait for any server to exit (both should run indefinitely)
    if let Some(result) = tasks.join_next().await {
        match result {
            Ok(server_result) => {
                if let Err(e) = server_result {
                    error!("Server exited with error: {:?}", e);
                    return Err(Box::new(e) as Box<dyn std::error::Error>);
                }
            }
            Err(e) => {
                error!("Server task panicked: {}", e);
                return Err(Box::new(Error::new_simple(
                    ErrorCode::ServerError,
                    format!("Server task panicked: {}", e),
                )) as Box<dyn std::error::Error>);
            }
        }
    }

    // We shouldn't reach here as the servers should run indefinitely
    info!("All server tasks have completed");
    Ok(())
}
