//! Event processor for managing event handlers and consumer setup

use std::sync::Arc;
use crate::shared::{EventType, EventHandlerRegistry};
use crate::infrastructure::{RedpandaClient};
use crate::domains::{BoundedContext1EventContextBuilder, BoundedContext2EventContextBuilder};
use tracing::{info, error};

/// Event processor that manages event handlers and consumer setup
pub struct EventProcessor {
    redpanda_client: Arc<RedpandaClient>,
    handler_registry: EventHandlerRegistry,
}

impl EventProcessor {
    /// Create a new event processor with default handlers
    pub fn new() -> Self {
        let redpanda_client = Arc::new(RedpandaClient::new());
        let mut handler_registry = EventHandlerRegistry::new();
        
        // Initialize domains with shared Redpanda client
        let (topic1, event_handlers_1) = BoundedContext1EventContextBuilder::new().build();
        let (topic2, event_handlers_2) = BoundedContext2EventContextBuilder::new().build();
        
        // TODO: Fix this!
        handler_registry.register_handler(topic1, event_handlers_1);
        handler_registry.register_handler(topic2, event_handlers_2);

        Self {
            redpanda_client,
            handler_registry,
        }
    }

    /// Start consuming events from all configured event types
    pub async fn start_consumers(&self) -> Result<(), String> {
        let default_event_types = EventType::all();
        
        if let Err(e) = self.redpanda_client.start_multi_event_consumer(
            default_event_types, 
            self.handler_registry.clone()
        ).await {
            error!("Failed to start continuous consumers: {}", e);
            return Err(e);
        }
        
        info!("Continuous consumers started successfully");
        Ok(())
    }
}
