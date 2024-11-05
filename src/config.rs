use serde::Deserialize;
use shellexpand;
use std::fs;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct NodeConfig {
    pub id: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub nodes: Vec<NodeConfig>,
}

pub fn load_config(file_path: &str) -> Config {
    let config_content = fs::read_to_string(file_path).expect("Failed to read config file");
    let expanded_content = shellexpand::env(&config_content).unwrap().to_string();
    serde_yaml::from_str(&expanded_content).expect("Failed to parse config")
}
