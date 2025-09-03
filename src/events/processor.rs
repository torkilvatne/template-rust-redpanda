//! Event processor for managing event handlers and consumer setup

use std::sync::Arc;
use crate::shared::{Topic, EventHandlerRegistry, EventHandler, Event};
use crate::infrastructure::{RedpandaClient};
use crate::domains::{OrderEventContextBuilder, LogisticsEventContextBuilder};
use tracing::{info};

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
        
        // Initialize domains and gather handlers
        let order_handlers: Vec<(Topic, EventHandler)> = OrderEventContextBuilder::new().build();
        let logistics_handlers: Vec<(Topic, EventHandler)> = LogisticsEventContextBuilder::new().build();

        for (event_type, handler) in order_handlers.into_iter() {
            handler_registry.register_handler(event_type, handler);
        }
        for (event_type, handler) in logistics_handlers.into_iter() {
            handler_registry.register_handler(event_type, handler);
        }

        Self {
            redpanda_client,
            handler_registry,
        }
    }

    /// Start consuming events from all configured event types
    pub async fn start_consumers(&self) -> Result<(), String> {
        for event_type in Topic::all().into_iter() {
            if let Some(handlers) = self.handler_registry.get_handlers(&event_type) {
                let fan_out = {
                    let handlers = handlers.clone();
                    Arc::new(move |event: &Event| {
                        for h in &handlers {
                            h(event);
                        }
                    }) as EventHandler
                };
                self.redpanda_client.start_event_consumer(event_type, Some(fan_out)).await?;
            } else {
                // No handlers registered for this topic; still start with None or skip
                self.redpanda_client.start_event_consumer(event_type, None).await?;
            }
        }
        info!("Continuous consumers started successfully");
        Ok(())
    }
}
