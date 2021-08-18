use super::Payload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub entity: String,
    pub account_id: String,
    pub event: String,
    pub contains: Option<Vec<String>>,
    pub payload: Payload,
    pub created_at: i64,
}
