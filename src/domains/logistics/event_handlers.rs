use crate::shared::{LogisticsEvent};
use tracing::{debug};

pub struct LogisticsEventHandlers;

impl LogisticsEventHandlers {
    pub fn new() -> Self {
        Self
    }   
    
    pub async fn handle_create_event(&self, event: &LogisticsEvent) {
        debug!("Event handler processing Logistics create event: {:?}", event);
    
    }
    
    pub async fn handle_update_event(&self, event: &LogisticsEvent) {
        debug!("Event handler processing Logistics update event: {:?}", event);
    }
    
}