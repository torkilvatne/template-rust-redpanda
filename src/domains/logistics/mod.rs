use crate::shared::{Topic, EventHandler};

pub mod topic_handler;
pub mod event_handlers;
pub mod models;
pub mod api;

use topic_handler::*;

pub struct LogisticsEventContextBuilder {}

impl LogisticsEventContextBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> Vec<(Topic, EventHandler)> {
        let handler = LogisticsTopicHandler::new();
        handler.get_topic_handlers()
    }
}