use crate::shared::{Topic, EventHandler};
// use std::sync::Arc; // no longer needed

pub mod topic_handler;
pub mod event_handlers;
pub mod models;
pub mod api;

use topic_handler::*;

pub struct OrderEventContextBuilder {}

impl OrderEventContextBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> Vec<(Topic, EventHandler)> {
        let handler = OrderTopicHandler::new();
        handler.get_topic_handlers()
    }
}
