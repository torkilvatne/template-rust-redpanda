use crate::shared::{Event, EventPayload, OrderEvent, LogisticsEvent, LogisticsActions, Topic};
use crate::infrastructure::{RedpandaClient};
use tracing::{debug, error};
use chrono::Utc;
use uuid::Uuid;

pub struct OrderEventHandlers;

impl OrderEventHandlers {
    pub fn new() -> Self {
        Self
    }   
    
    pub async fn handle_create_event(&self, event: &Event) {
        debug!("Event handler processing Order create event: {:?}", event);
        
        // Only create follow-up when the source is an Order event
        if let EventPayload::OrderEvent(source) = &event.payload {
            debug!("This event should pretend to trigger another event after consuming this.");
            let followup_event = Event {
                id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                event_type: Topic::Logistics,
                payload: EventPayload::LogisticsEvent(LogisticsEvent {
                    action: LogisticsActions::Created,
                    message: source.message.clone(),
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
    }
    
    pub async fn handle_update_event(&self, event: &OrderEvent) {
        debug!("Event handler processing Order update event: {:?}", event);
    }
    
}