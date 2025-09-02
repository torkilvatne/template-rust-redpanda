//! Configuration management for the application

use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub server_port: String,
    pub redpanda_brokers: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "{{server_port}}".to_string()),
            redpanda_brokers: env::var("REDPANDA_BROKERS").unwrap_or_else(|_| "localhost:9093".to_string()),
        }
    }
    
    /// Get the bind address for the server
    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.server_port)
    }
}



