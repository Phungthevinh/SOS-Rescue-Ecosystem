use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub phone: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>,
    pub pin_hash: Option<String>,
    pub voice_keyword: Option<String>,
    pub is_verified: Option<bool>,
    pub trust_score: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
