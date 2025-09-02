use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoundedContext1Data {
    pub id: String,
    pub name: String,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoundedContext1Config {
    pub enabled: bool,
    pub max_retries: u32,
}
