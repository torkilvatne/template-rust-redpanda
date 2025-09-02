use crate::shared::{EventType, EventHandler};
use std::sync::Arc;

pub mod topic_handler;
pub mod models;
pub mod api;

use topic_handler::*;

pub struct BoundedContext2EventContextBuilder {
    pub topic: EventType,
}

impl BoundedContext2EventContextBuilder {
    pub fn new() -> Self {
        Self {
            topic: EventType::BoundedContext2,
        }
    }

    pub fn build(self) -> (EventType, EventHandler) {
        (
            self.topic,
            Arc::new(bounded_context_2_topic_handler),
        )
    }
}
