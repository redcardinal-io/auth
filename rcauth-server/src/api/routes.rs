use crate::api::health::health_check;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}
