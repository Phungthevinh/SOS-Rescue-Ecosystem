use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SosEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub status: String,
    pub trigger_method: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}
