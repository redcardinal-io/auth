use crate::routes::{logger, HealthCheckDoc};
use axum::Router;
use std::error::Error;
use tracing::{info, warn};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use std::net::SocketAddr;
use std::str::FromStr;
use tower_http::cors::{Any, CorsLayer};
use utoipa::openapi::{Info, Paths};

use crate::Config;

/// Starts the main API HTTP server with configured routes, CORS, and optional Swagger UI documentation.
///
/// Validates the provided configuration, applies CORS settings if enabled, and sets up API routes under `/api/v1`.
/// If Swagger UI is enabled, serves OpenAPI documentation at `/swagger-ui` and `/api/v1/api-docs/openapi.json`.
/// Binds to the address specified in the configuration and serves requests asynchronously.
///
/// # Errors
///
/// Returns an error if the configuration is invalid or if the server fails to bind or run.
///
/// # Examples
///
/// ```no_run
/// let config = Config::default();
/// tokio::runtime::Runtime::new().unwrap().block_on(async {
///     run_api_server(&config).await.unwrap();
/// });
/// ```
pub async fn run_api_server(config: &Config) -> Result<(), Box<dyn Error>> {
    if let Err(err) = config.validate() {
        return Err(format!("Invalid API server configuration: {}", err).into());
    }

    let mut app = axum::Router::new();

    if config.enable_cors {
        let cors = if config.cors_allowed_origins.contains(&"*".to_string()) {
            warn!(
                "CORS is configured to allow any origin for api server. This is not recommended for production."
            );
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
                .expose_headers(Any)
        } else {
            let origins = config
                .cors_allowed_origins
                .iter()
                .map(|origin| origin.parse().expect("Invalid CORS origin"))
                .collect::<Vec<_>>();

            CorsLayer::new()
                .allow_origin(origins)
                .allow_methods(Any)
                .allow_headers(Any)
                .expose_headers(Any)
        };

        app = app.layer(cors);
    }

    // Setup OpenAPI documentation if enabled
    let mut app = if config.enable_swagger {
        info!(
            "Enabling Swagger UI at /swagger-ui and OpenAPI docs at /api/v1/api-docs/openapi.json"
        );
        let mut openapi =
            utoipa::openapi::OpenApi::new(Info::new("RCAuth API", "0.0.1"), Paths::new());

        openapi.merge(HealthCheckDoc::openapi());

        app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
    } else {
        app
    };

    let routes = Router::new().merge(crate::routes::routes());
    app = app
        .nest("/api/v1", routes)
        .layer(logger::create_logger_middleware_http());

    let addr = config.api_addr();
    let socket_addr = SocketAddr::from_str(&addr).expect("Invalid address");

    info!(addr = %addr, "🚀 Starting API server");

    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Starts the management HTTP server with the specified configuration.
///
/// Validates the configuration, sets up CORS and optional Swagger UI documentation, nests management routes under `/management/v1`, and serves requests on the configured address.
///
/// # Errors
///
/// Returns an error if the configuration is invalid or if the server fails to bind or run.
///
/// # Examples
///
/// ```no_run
/// let config = Config::default();
/// tokio::spawn(async move {
///     run_management_server(&config).await.unwrap();
/// });
/// ```
pub async fn run_management_server(config: &Config) -> Result<(), Box<dyn Error>> {
    if let Err(err) = config.validate() {
        return Err(format!("Invalid API server configuration: {}", err).into());
    }

    let mut app = axum::Router::new();

    if config.enable_cors {
        let cors = if config.cors_allowed_origins.contains(&"*".to_string()) {
            warn!(
                "CORS is configured to allow any origin for management server. This is not recommended for production."
            );
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
                .expose_headers(Any)
        } else {
            let origins = config
                .cors_allowed_origins
                .iter()
                .map(|origin| origin.parse().expect("Invalid CORS origin"))
                .collect::<Vec<_>>();

            CorsLayer::new()
                .allow_origin(origins)
                .allow_methods(Any)
                .allow_headers(Any)
                .expose_headers(Any)
        };

        app = app.layer(cors);
    }

    // Setup OpenAPI documentation if enabled

    let mut app = if config.enable_swagger {
        let mut openapi = utoipa::openapi::OpenApi::new(
            Info::new("RCAuth Management API", "0.0.1"),
            Paths::new(),
        );

        openapi.merge(HealthCheckDoc::openapi());

        app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
    } else {
        app
    };

    let routes = Router::new().merge(crate::routes::routes());
    app = app
        .nest("/management/v1", routes)
        .layer(logger::create_logger_middleware_http());

    let addr = config.management_addr();
    let socket_addr = SocketAddr::from_str(&addr).expect("Invalid address");

    info!(addr = %addr, "🚀 Starting API server");

    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
