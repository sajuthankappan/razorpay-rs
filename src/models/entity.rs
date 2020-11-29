use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Entity<T> {
    entity: T,
}
