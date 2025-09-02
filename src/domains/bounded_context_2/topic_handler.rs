use crate::shared::{Event, EventPayload, BoundedContext2Event, BoundedContext2Actions};
use tracing::{debug, error, info};

pub struct BoundedContext2EventHandlers;

impl BoundedContext2EventHandlers {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn handle_create_event(&self, event: &BoundedContext2Event) {
        debug!("🔄 Processing BoundedContext2 create event: {}", event.message);
    
    }
    
    pub async fn handle_update_event(&self, event: &BoundedContext2Event) {
        debug!("🔄 Processing BoundedContext2 update event: {:?}", event);
    }
    
}

pub fn bounded_context_2_topic_handler(event: &Event) {
    debug!("�� Processing BoundedContext2 event: {:?}", event);
    
    // Clone what we need for the async task
    let event_clone = event.clone();
    
    // Spawn async work in background
    tokio::spawn(async move {
        let bounded_context_2_event_handlers = BoundedContext2EventHandlers::new();
        
        match &event_clone.payload {
            EventPayload::BoundedContext2Event(event) => match event.action {
                BoundedContext2Actions::Created => {
                    info!("Processing bounded context 2 create event");
                    bounded_context_2_event_handlers.handle_create_event(event).await
                }
                BoundedContext2Actions::Updated => {
                    info!("Processing bounded context 2 update event");
                    bounded_context_2_event_handlers.handle_update_event(event).await
                }
            },
            _ => {
                error!("Unexpected event payload type for bounded context 2");
            }
        }
    });
}