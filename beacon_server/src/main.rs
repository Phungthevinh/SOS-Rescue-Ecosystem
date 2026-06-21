use sqlx::postgres::PgPoolOptions;
use tracing::info;
pub mod app_state;
pub mod config;
pub mod errors;
pub mod handlers;
pub mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    // 1. Khởi tạo Config (Đọc từ environment)
    let config = config::AppConfig::load();

    // 2. Connect Database
    info!("Connecting to database...");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    info!("✅ Database connected successfully!");

    // 3. Khởi tạo App State
    let app_state = app_state::AppState { db: db_pool };

    // 4. Tạo Router
    let app = routes::create_router(app_state);

    // 5. Khởi động Server
    let addr = format!("0.0.0.0:{}", config.port);

    info!("🚀 Beacon Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
