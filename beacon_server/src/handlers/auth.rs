use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub phone: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    // 1. Kiểm tra User trong database
    let existing_user = sqlx::query!(
        r#"
        SELECT id, is_verified FROM users WHERE phone = $1
        "#,
        payload.phone
    )
    .fetch_optional(&state.db)
    .await?;

    // 2. Phân nhánh xử lý User
    if let Some(user) = existing_user {
        // NẾU: User đã có VÀ đã xác minh -> Báo lỗi ngay lập tức
        if user.is_verified.unwrap_or(false) {
            return Err(AppError::BadRequest(
                "Số điện thoại đã được đăng ký. Vui lòng đăng nhập.".to_string(),
            ));
        }
        // NẾU: User đã có NHƯNG chưa xác minh -> Không làm gì cả (thả trôi xuống bước 3)
    } else {
        // NẾU: User chưa từng tồn tại -> Insert mới
        sqlx::query!(
            r#"
            INSERT INTO users (phone, is_verified)
            VALUES ($1, false)
            "#,
            payload.phone
        )
        .execute(&state.db)
        .await?;
    }

    // 3. LOGIC CHUNG: Tạo OTP (Code chạy đến đây nghĩa là 100% hợp lệ để tạo OTP)
    let otp_code = "123456".to_string();

    sqlx::query!(
        "INSERT INTO otp_codes (phone, code, expires_at) VALUES ($1, $2, NOW() + INTERVAL '5 minutes')",
        payload.phone,
        otp_code
    )
    .execute(&state.db)
    .await?;

    // 4. LOGIC CHUNG: Trả về kết quả thành công (Chỉ viết 1 lần duy nhất ở cuối hàm)
    Ok(Json(RegisterResponse {
        message: "Mã OTP đã được gửi đến số điện thoại của bạn".to_string(),
    }))
}
