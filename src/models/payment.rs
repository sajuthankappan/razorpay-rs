use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use super::Card;

#[derive(Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Payment {
    id: String,
    entity: String,
    amount: i32,
    currency: String,
    base_amount: Option<i32>,
    status: String,
    order_id: String,
    international: bool,
    method: String,
    amount_refunded: i32,
    refund_status: Option<String>,
    captured: bool,
    description: String,
    card_id: Option<String>,
    card: Option<Card>,
    invoice_id: Option<String>,
    email: String,
    contact: String,
    notes: Option<Vec<String>>,
    fee: Option<i32>,
    tax: Option<i32>,
    error_code: Option<String>,
    error_description: Option<String>,
    error_source: Option<String>,
    error_step: Option<String>,
    error_reason: Option<String>,
    acquirer_data: Option<AcquirerData>,
    created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct AcquirerData {
    auth_code: Option<String>,
}