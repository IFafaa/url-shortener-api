use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;

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
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Failed to bind to address");

    let app = routes::router::create_router(db_pool);

    println!("🚀 Server started successfully");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
