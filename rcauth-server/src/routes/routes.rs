#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check passed", body = String)
    )
)]
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

pub fn routes() -> axum::Router {
    axum::Router::new().route("/health", axum::routing::get(health_check))
}
