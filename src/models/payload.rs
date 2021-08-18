use super::{Entity, Order, Payment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub payment: Option<Entity<Payment>>,
    pub order: Option<Entity<Order>>,
}
