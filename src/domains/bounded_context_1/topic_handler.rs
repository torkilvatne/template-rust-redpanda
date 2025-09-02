use crate::shared::{Event, EventPayload, BoundedContext1Event, BoundedContext2Event, BoundedContext1Actions,  BoundedContext2Actions, EventType};
use crate::infrastructure::{RedpandaClient};
use tracing::{debug, error, info};
use chrono::Utc;
use uuid::Uuid;

pub struct BoundedContext1EventHandlers;

impl BoundedContext1EventHandlers {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn handle_create_event(&self, event: &BoundedContext1Event) {
        debug!("🔄 Processing BoundedContext1 create event: {}", event.message);
        
        debug!("This event should pretend to trigger another event after consuming this.");
        let followup_event = Event {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: EventType::BoundedContext2,
            payload: EventPayload::BoundedContext2Event(BoundedContext2Event {
                action: BoundedContext2Actions::Created,
                message: event.message.clone(),
            }),
        };

        let redpanda_client = RedpandaClient::new();
        
        match redpanda_client.send_event(&followup_event).await {
            Ok(_) => {
                debug!("Pretend-followup event send!");
            },
            Err(e) => {
                error!("Failed to send followup event: {}", e);
            }
        };
    }
    
    pub async fn handle_update_event(&self, event: &BoundedContext1Event) {
        debug!("🔄 Processing BoundedContext1 update event: {:?}", event);
    }
    
}

pub fn bounded_context_1_topic_handler(event: &Event) {
    debug!("Processing BoundedContext1 event: {:?}", event);
    
    // Clone what we need for the async task
    let event_clone = event.clone();
    
    // Spawn async work in background
    tokio::spawn(async move {
        let bounded_context_1_event_handlers = BoundedContext1EventHandlers::new();
        
        match &event_clone.payload {
            EventPayload::BoundedContext1Event(event) => match event.action {
                BoundedContext1Actions::Created => {
                    info!("Processing bounded context 1 create event");
                    bounded_context_1_event_handlers.handle_create_event(event).await
                }
                BoundedContext1Actions::Updated => {
                    info!("Processing bounded context 1 update event");
                    bounded_context_1_event_handlers.handle_update_event(event).await
                }
            },
            _ => {
                error!("Unexpected event payload type for bounded context 1");
            }
        }
    });
}