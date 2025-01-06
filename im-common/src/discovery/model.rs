use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EndpointInfo {
    pub ip: String,
    pub port: String,
    pub metadata: Option<Value>,
}

impl EndpointInfo {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        serde_json::from_slice(data)
    }
}
