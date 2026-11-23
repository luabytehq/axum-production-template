use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub balance: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountPayload {
    pub username: String,
    pub initial_balance: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AdjustBalancePayload {
    pub amount: i64,
}
