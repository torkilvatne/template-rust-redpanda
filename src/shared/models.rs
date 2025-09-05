{% if use_redpanda %}
use serde::{Deserialize, Serialize};
{% endif %}
{% if use_redpanda == false %}
use serde::{Serialize};
{% endif %}
{% if use_redpanda %}
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::collections::HashMap;
// Topics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Topic {
    Order,
    Logistics,
}

impl Topic {
    pub fn as_str(&self) -> &'static str {
        match self {
            Topic::Order => "order",
            Topic::Logistics => "logistics",
        }
    }

    pub fn all() -> Vec<Topic> {
        vec![
            Topic::Order,
            Topic::Logistics,
        ]
    }
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// Events
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: Topic,
    pub payload: EventPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventPayload {
    OrderEvent(OrderEvent),
    LogisticsEvent(LogisticsEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderEvent {
    pub action: OrderActions,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogisticsEvent {
    pub action: LogisticsActions,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderActions {
    Created,
    Updated,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogisticsActions {
    Created,
    Updated,
}

// Event handler registry
pub type EventHandler = Arc<dyn Fn(&Event) + Send + Sync>;

#[derive(Clone)]
pub struct EventHandlerRegistry {
    handlers: Arc<HashMap<Topic, Vec<EventHandler>>>,
}

impl EventHandlerRegistry {
    pub fn new() -> Self {
        Self { handlers: Arc::new(HashMap::new()) }
    }

    pub fn register_handler(&mut self, event_type: Topic, handler: EventHandler) {
        let mut handlers = HashMap::clone(&self.handlers);
        handlers.entry(event_type).or_insert_with(Vec::new).push(handler);
        self.handlers = Arc::new(handlers);
    }

    pub fn get_handlers(&self, event_type: &Topic) -> Option<&Vec<EventHandler>> {
        self.handlers.get(event_type)
    }
}
{% endif %}
// API

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub message: String,
    pub data: Option<serde_json::Value>,
}
