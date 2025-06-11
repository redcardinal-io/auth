use rcauth_core::error::{Error, ErrorCode, Result};
use tracing::info;

pub async fn run() -> Result<()> {
    // TODO: Use server configuration to load environment variables
    info!("Server started at http://0.0.0.0:8000");
    rcauth_server::server::build_http_server("0.0.0.0".to_string(), 8000)
        .await
        .map_err(|e| {
            tracing::error!("Failed to start server: {}", e);
            Error::new_simple(ErrorCode::Internal, "Failed to start server")
        })?;
    Ok(())
}
