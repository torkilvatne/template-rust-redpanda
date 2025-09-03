use crate::domains::order::event_handlers::OrderEventHandlers;
use crate::shared::{Event, EventPayload, OrderActions, Topic, EventHandler, LogisticsActions};
use tracing::{debug, error, info};
use std::sync::Arc;

pub struct OrderTopicHandler;

impl OrderTopicHandler {
    pub fn new() -> Self {
        Self
    }
}

impl OrderTopicHandler {
    pub fn get_topic_handlers(&self) -> Vec<(Topic, EventHandler)> {
        vec![
            (Topic::Order, Arc::new(order_topic_handler)),
            (Topic::Logistics, Arc::new(order_logistics_topic_handler)),
        ]
    }
}

pub fn order_topic_handler(event: &Event) {
    debug!("Order topic handler received Order event: {:?}", event);
    
    // Clone what we need for the async task
    let event_clone = event.clone();
    
    // Spawn async work in background
    tokio::spawn(async move {
        let order_event_handlers = OrderEventHandlers::new();
        
        match &event_clone.payload {
            EventPayload::OrderEvent(event) => match event.action {
                OrderActions::Created => {
                    // Pass full Event to generic handler
                    order_event_handlers.handle_create_event(&event_clone).await
                }
                OrderActions::Updated => {
                    order_event_handlers.handle_update_event(event).await
                }
            },
            _ => {
                error!("Unexpected event payload type for Order");
            }
        }
    });
}

pub fn order_logistics_topic_handler(event: &Event) {
    debug!("Order topic handler received Logistics event: {:?}", event);
    
    // Clone what we need for the async task
    let event_clone = event.clone();
    
    tokio::spawn(async move {
    
        match &event_clone.payload {
            EventPayload::LogisticsEvent(logistics_event) => match logistics_event.action {
                LogisticsActions::Created => {
                    info!("Processing Logistics create event in Order domain");
                }
                LogisticsActions::Updated => {
                    info!("Processing Logistics update event in Order domain");
                }
            }
            _ => {
                error!("Unexpected event payload type for Logistics in Order domain");
            }
        }
    });
}