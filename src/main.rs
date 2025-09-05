use std::net::SocketAddr;
use std::str::FromStr;
{% if use_redpanda %}
use tracing::{debug, info, error};
{% endif %}
{% if use_redpanda == false %}
use tracing::{debug, info};
{% endif %}
use config::Config;

mod api;
mod config;
mod domains;
{% if use_redpanda %}
mod events;
mod infrastructure;
{% endif %}
mod shared;

use api::routes::create_router;
{% if use_redpanda %}
use events::processor::EventProcessor;
{% endif %}

#[tokio::main]
async fn main() {
    // Initialize logging with debug level
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    // Load configuration
    let config = Config::from_env();
    debug!("Configuration loaded: {:?}", config);

    {% if use_redpanda %}
    // Initialize event processor with shared Redpanda client
    let event_processor = EventProcessor::new();
    
    // Start event consumers
    if let Err(e) = event_processor.start_consumers().await {
        error!("Failed to start event consumers: {}", e);
        std::process::exit(1);
    }
    {% endif %}

    // Create router
    let app = create_router();

    // Run it
    let bind_address = config.bind_address();
    let addr = SocketAddr::from_str(&bind_address).unwrap();
    info!("Server starting on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
