pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
}

impl AppConfig {
    // Hàm dùng để load các thông số cấu hình từ biến môi trường
    pub fn load() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env");
            
        let port_str = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let port: u16 = port_str.parse().expect("PORT must be a valid number");

        AppConfig {
            database_url,
            port,
        }
    }
}
