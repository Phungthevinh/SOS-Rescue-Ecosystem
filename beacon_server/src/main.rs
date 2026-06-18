use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

async fn health_check() -> &'static str {
    // Trả về một chuỗi đơn giản
    "OK - Beacon Server is running!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    // 2. Connect Database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    info!("Connecting to database...");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    info!("✅ Database connected successfully!");

    // 3. Khởi tạo App State
    let app_state = AppState { db: db_pool };

    // 4. Tạo Router và Inject State
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(app_state);

    // 5. Khởi động Server
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    info!("🚀 Beacon Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
