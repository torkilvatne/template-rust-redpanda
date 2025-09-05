{% if use_redpanda %}
use crate::shared::{Topic, EventHandler};

pub mod topic_handler;
pub mod event_handlers;
{% endif %}
pub mod models;
pub mod api;

{% if use_redpanda %}
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
{% endif %}
