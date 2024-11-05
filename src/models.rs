use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub ip: String,
    pub port: u16,
}
