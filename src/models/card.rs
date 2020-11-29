use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Card {
    id: String,
    entity: String,
    name: String,
    last4: String,
    network: String,
    #[serde(rename = "type")]
    card_type: String,
    issuer: Option<String>,
    international: bool,
    emi: bool,
    sub_type: Option<String>,
}
