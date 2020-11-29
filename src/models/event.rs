use super::Payload;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Event {
    entity: String,
    account_id: String,
    event: String,
    contains: Option<Vec<String>>,
    payload: Payload,
    created_at: i64,
}
