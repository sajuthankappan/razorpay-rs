use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
//use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Order {
    id: String,
    entity: String,
    amount: i32,
    amount_paid: i32,
    amount_due: i32,
    currency: String,
    receipt: String,
    status: String,
    offer_id: Option<String>,
    attempts: i32,
    //notes: Vec<Value>,
    created_at: i64,
}
