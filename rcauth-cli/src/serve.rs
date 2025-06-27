use rcauth_core::error::{Error, ErrorCode};
use tokio::task::JoinSet;
use tracing::{error, info};

use crate::config;

/// Starts and manages the authentication API and management servers concurrently.
///
/// Loads the server configuration, then launches both the API server and the management server as asynchronous tasks. Waits for either server to exit, handling errors and panics by logging and returning them as boxed errors. Both servers are expected to run indefinitely; reaching the end of this function indicates all server tasks have completed unexpectedly.
///
/// # Returns
///
/// Returns `Ok(())` if both servers run without error (which is not expected), or a boxed error if a server fails or panics.
///
/// # Examples
///
/// ```no_run
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     rcauth_cli::serve::run().await
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
