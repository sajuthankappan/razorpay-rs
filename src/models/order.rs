use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Order {
    id: String,
    entity: String,
    amount: i32,
    amount_paid: i32,
    amount_due: i32,
    currency: String,
    receipt: Option<String>,
    offer_id: Option<String>,
    status: String,
    attempts: i32,
    notes: Vec<String>,
    created_at: i64,
}
