use super::{Entity, Order, Payment};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Payload {
    payment: Option<Entity<Payment>>,
    order: Option<Entity<Order>>,
}
