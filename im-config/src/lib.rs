use std::{fs, path::PathBuf};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub global: GlobalConfig,
    pub ip_config: IpConfig,
    pub discovery: DiscoveryConfig,
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub env: String,
}

#[derive(Debug, Deserialize)]
pub struct IpConfig {
    pub service_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiscoveryConfig {
    pub endpoints: Vec<String>,
    pub timeout: u64,
}

impl AppConfig {
    pub fn load(file_path: PathBuf) -> Result<Self> {
        let config_content = fs::read_to_string(file_path)?;
        let config: AppConfig = serde_yaml::from_str(&config_content)?;
        Ok(config)
    }
}
