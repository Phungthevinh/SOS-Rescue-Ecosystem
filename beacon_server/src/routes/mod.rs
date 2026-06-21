use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;
use crate::handlers;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/register", post(handlers::auth::register_handler))
        .with_state(state)
}
