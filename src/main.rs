use axum::http;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod routes {
    pub mod router;
}

pub mod handlers {
    pub mod get_original_url_handler;
    pub mod shortener_url_handler;
}

pub mod dtos {
    pub mod shortener_url_dto;
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_headers([http::header::CONTENT_TYPE]);

    let app = routes::router::create_router(db_pool).layer(cors);

    println!("🚀 Server started successfully");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
