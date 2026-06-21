use axum::routing::get;
use axum::Router;

use crate::app_state::AppState;
use crate::handlers;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .with_state(state)
}
