use crate::domains::logistics::event_handlers::LogisticsEventHandlers;
use crate::shared::{Event, EventPayload, LogisticsActions, Topic, EventHandler};
use std::sync::Arc;
use tracing::{debug, error};

pub struct LogisticsTopicHandler;

impl LogisticsTopicHandler {
    pub fn new() -> Self {
        Self
    }
}

impl LogisticsTopicHandler {
    pub fn get_topic_handlers(&self) -> Vec<(Topic, EventHandler)> {
        vec![
            (Topic::Logistics, Arc::new(logistics_topic_handler)),
        ]
    }
}

pub fn logistics_topic_handler(event: &Event) {
    debug!("Logistics topic handler received Logistics event: {:?}", event);
    
    // Clone what we need for the async task
    let event_clone = event.clone();
    
    // Spawn async work in background
    tokio::spawn(async move {
        let logistics_event_handlers = LogisticsEventHandlers::new();
        
        match &event_clone.payload {
            EventPayload::LogisticsEvent(event) => match event.action {
                LogisticsActions::Created => {
                    logistics_event_handlers.handle_create_event(event).await
                }
                LogisticsActions::Updated => {
                    logistics_event_handlers.handle_update_event(event).await
                }
            },
            _ => {
                error!("Unexpected event payload type for Logistics");
            }
        }
    });
}