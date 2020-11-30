use super::Card;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Payment {
    id: String,
    entity: String,
    amount: i32,
    currency: String,
    status: String,
    base_amount: Option<i32>,
    base_currency: Option<String>,
    method: String,
    order_id: String,
    description: Option<String>,
    international: bool,
    refund_status: Option<String>,
    amount_refunded: i32,
    captured: bool,
    email: String,
    contact: String,
    fee: Option<i32>,
    tax: Option<i32>,
    error_code: Option<String>,
    error_description: Option<String>,
    error_source: Option<String>,
    error_step: Option<String>,
    error_reason: Option<String>,
    acquirer_data: Option<AcquirerData>,
    card_id: Option<String>,
    card: Option<Card>,
    invoice_id: Option<String>,
    notes: Vec<Value>,
    created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AcquirerData {
    auth_code: Option<String>,
}
