use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers;

use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/api/shortener",
            post(handlers::shortener_url_handler::execute),
        )
        .route("/api/short/{id}", get(handlers::get_original_url_handler::execute))
        .with_state(pool)
}
