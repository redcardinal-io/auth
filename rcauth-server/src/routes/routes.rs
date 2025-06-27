/// Handles the health check endpoint, returning a static "OK" response to indicate the service is operational.
///
/// # Examples
///
/// ```
/// let status = health_check().await;
/// assert_eq!(status, "OK");
/// ```
pub async fn health_check() -> &'static str {
    "OK"
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(health_check),
    tags(
        (name = "Health", description = "Health check endpoint")
    ),
    info(
        title = "RCAuth API",
        version = "0.0.1",
        description = "API for RCAuth authentication server"
    )
)]
pub struct HealthCheckDoc;

/// Creates an Axum router with the health check endpoint registered.
///
/// The returned router handles GET requests to `/health` by invoking the `health_check` handler.
///
/// # Examples
///
/// ```
/// let app = routes();
/// // You can now serve `app` using an Axum-compatible HTTP server.
/// ```
pub fn routes() -> axum::Router {
    axum::Router::new().route("/health", axum::routing::get(health_check))
}
