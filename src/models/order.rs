use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: String,
    pub entity: String,
    pub amount: i32,
    pub amount_paid: i32,
    pub amount_due: i32,
    pub currency: String,
    pub receipt: String,
    pub status: String,
    pub offer_id: Option<String>,
    pub attempts: i32,
    pub notes: Option<Value>,
    pub created_at: i64,
}
