mod api;
mod management;
mod middleware;

pub use middleware::*;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = String)
    ),
    tag = "Health"
)]
pub async fn health_check() -> &'static str {
    "OK"
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(health_check),
    tags(
        (name = "Health", description = "System health and status endpoints")
    ),
    info(
        title = "RedCardinal Authentication API",
        version = "0.1.0",
        description = "Authentication and authorization service for the RedCardinal platform",
    )
)]
pub struct HealthCheckDoc;

pub fn routes() -> axum::Router {
    axum::Router::new().route("/health", axum::routing::get(health_check))
}
