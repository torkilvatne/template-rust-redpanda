use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub payload: EventPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventPayload {
    BoundedContext1Event(BoundedContext1Event),
    BoundedContext2Event(BoundedContext2Event),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    BoundedContext1,
    BoundedContext2,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::BoundedContext1 => "bounded-context-1",
            EventType::BoundedContext2 => "bounded-context-2",
        }
    }

    pub fn all() -> Vec<EventType> {
        vec![
            EventType::BoundedContext1,
            EventType::BoundedContext2,
        ]
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundedContext1Event {
    pub action: BoundedContext1Actions,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundedContext2Event {
    pub action: BoundedContext2Actions,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoundedContext1Actions {
    Created,
    Updated,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoundedContext2Actions {
    Created,
    Updated,
}

// Event handler registry
pub type EventHandler = Arc<dyn Fn(&Event) + Send + Sync>;

#[derive(Clone)]
pub struct EventHandlerRegistry {
    handlers: Arc<HashMap<EventType, EventHandler>>,
}

impl EventHandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(HashMap::new()),
        }
    }

    pub fn register_handler(&mut self, event_type: EventType, handler: EventHandler) {
        let mut handlers = HashMap::clone(&self.handlers);
        handlers.insert(event_type, handler);
        self.handlers = Arc::new(handlers);
    }

    pub fn get_handler(&self, event_type: &EventType) -> Option<&EventHandler> {
        self.handlers.get(event_type)
    }
}

// API

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub message: String,
    pub data: Option<serde_json::Value>,
}
