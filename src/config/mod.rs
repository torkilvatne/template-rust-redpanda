use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_port: String,
    {% if use_redpanda %}
    pub redpanda_brokers: String,
    {% endif %}
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "{{server_port}}".to_string()),
            {% if use_redpanda %}
            redpanda_brokers: env::var("REDPANDA_BROKERS").unwrap_or_else(|_| "localhost:9093".to_string()),
            {% endif %}
        }
    }
    
    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.server_port)
    }
}



