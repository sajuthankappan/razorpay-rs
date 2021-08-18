use super::Card;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: String,
    pub entity: String,
    pub amount: i32,
    pub currency: String,
    pub status: String,
    pub base_amount: Option<i32>,
    pub base_currency: Option<String>,
    pub method: String,
    pub order_id: String,
    pub description: Option<String>,
    pub international: bool,
    pub refund_status: Option<String>,
    pub amount_refunded: i32,
    pub captured: bool,
    pub email: String,
    pub contact: String,
    pub fee: Option<i32>,
    pub tax: Option<i32>,
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    pub error_source: Option<String>,
    pub error_step: Option<String>,
    pub error_reason: Option<String>,
    pub acquirer_data: Option<AcquirerData>,
    pub card_id: Option<String>,
    pub card: Option<Card>,
    pub invoice_id: Option<String>,
    pub notes: Option<Value>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AcquirerData {
    auth_code: Option<String>,
}
