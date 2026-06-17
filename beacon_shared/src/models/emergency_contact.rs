use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct EmergencyContact {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub phone: String,
    pub relationship: Option<String>,
    pub priority: i16,
    pub created_at: DateTime<Utc>,
}
