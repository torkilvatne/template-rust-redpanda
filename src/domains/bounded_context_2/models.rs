use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoundedContext2Data {
    pub id: String,
    pub status: String,
    pub priority: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoundedContext2Config {
    pub enabled: bool,
    pub timeout_seconds: u64,
}



