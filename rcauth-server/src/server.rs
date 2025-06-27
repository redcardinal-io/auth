use crate::routes::HealthCheckDoc;
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
    let routes = Router::new().merge(crate::routes::routes());

    let routes = if config.enable_swagger {
        let mut openapi =
            utoipa::openapi::OpenApi::new(Info::new("RCAuth API", "0.0.1"), Paths::new());

        openapi.merge(HealthCheckDoc::openapi());

        routes.merge(SwaggerUi::new("/swagger-ui").url("/api/v1/api-docs/openapi.json", openapi))
    } else {
        routes
    };

    app = app
        .nest("/api/v1", routes)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = config.api_addr();
    let socket_addr = SocketAddr::from_str(&addr).expect("Invalid address");

    info!(addr = %addr, "🚀 Starting API server");

    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Run the management server with the provided configuration
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
    let routes = Router::new().merge(crate::routes::routes());

    let routes = if config.enable_swagger {
        let mut openapi = utoipa::openapi::OpenApi::new(
            Info::new("RCAuth Management API", "0.0.1"),
            Paths::new(),
        );

        openapi.merge(HealthCheckDoc::openapi());

        routes.merge(SwaggerUi::new("/swagger-ui").url("/api/v1/api-docs/openapi.json", openapi))
    } else {
        routes
    };

    app = app
        .nest("/management/v1", routes)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = config.management_addr();
    let socket_addr = SocketAddr::from_str(&addr).expect("Invalid address");

    info!(addr = %addr, "🚀 Starting API server");

    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
