use crate::shared::models::{Event, Topic, EventHandler};
use crate::config::Config;
use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
    consumer::{Consumer, StreamConsumer},
    Message as KafkaMessage,
};
use std::time::Duration;
use tracing::{debug, info, warn};

#[derive(Clone)]
pub struct RedpandaClient {
    brokers: String,
}

impl RedpandaClient {
    pub fn new() -> Self {
        Self {
            brokers: Config::from_env().redpanda_brokers,
        }
    }

    pub async fn send_event(&self, event: &Event) -> Result<(), String> {
        let payload = serde_json::to_string(&event)
            .map_err(|e| format!("Failed to serialize event: {}", e))?;

        let topic_str = event.event_type.as_str();
        debug!("Sending event to topic: {}", topic_str);

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .map_err(|e| format!("Failed to create producer: {}", e))?;

        let record = FutureRecord::to(topic_str)
            .payload(payload.as_bytes())
            .key("rust-server");

        match producer.send(record, Duration::from_secs(5)).await {
            Ok(_) => {
                info!("Event sent successfully to topic: {}", topic_str);
                Ok(())
            }
            Err((e, _)) => Err(format!("Failed to send event: {}", e)),
        }
    }

    pub async fn start_event_consumer(&self, event_type: Topic, handler: Option<EventHandler>) -> Result<(), String> {
        let topic_str = event_type.as_str();
        debug!("Starting event consumer for event type: {}", topic_str);

        let brokers = self.brokers.clone();
        let event_type_clone = event_type;

        tokio::spawn(async move {
            if let Err(e) = Self::run_event_consumer(&brokers, &event_type_clone, handler).await {
                warn!("Event consumer error for event type {}: {}", event_type_clone, e);
            }
        });

        info!("Event consumer started for event type: {}", topic_str);
        Ok(())
    }

    async fn run_event_consumer(brokers: &str, event_type: &Topic, handler: Option<EventHandler>) -> Result<(), String> {
        let topic_str = event_type.as_str();

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", format!("rust-server-group-{}", topic_str))
            .set("auto.offset.reset", "earliest")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()
            .map_err(|e| format!("Failed to create consumer: {}", e))?;

        debug!("Subscribing to topic: {}", topic_str);
        consumer
            .subscribe(&[topic_str])
            .map_err(|e| format!("Failed to subscribe to topic: {:?}", e))?;

        info!("Starting to consume events from topic {}", topic_str);

        loop {
            match consumer.recv().await {
                Ok(message) => {
                    debug!("Received event from partition {} at offset {} in topic {}",
                        message.partition(), message.offset(), topic_str);

                    match message.payload_view::<str>() {
                        None => {
                            warn!("Received event with no payload from topic {}", topic_str);
                        }
                        Some(Ok(msg)) => {
                            info!("Event consumed from topic {}: {}", topic_str, msg);

                            // Try to parse as Event
                            if let Ok(event) = serde_json::from_str::<Event>(msg) {
                                debug!("Successfully parsed event from topic {}: {:?}", topic_str, event);

                                // Use the provided handler for this event type
                                if let Some(handler_fn) = &handler {
                                    handler_fn(&event);
                                } else {
                                    warn!("No handler provided for event type: {}", topic_str);
                                }
                            } else {
                                warn!("Failed to parse message as Event from topic {}: {}", topic_str, msg);
                            }
                        }
                        Some(Err(e)) => {
                            warn!("Error parsing event from topic {}: {}", topic_str, e);
                        }
                    }
                }
                Err(e) => {
                    if e.to_string().contains("UnknownTopicOrPartition") {
                        debug!("Topic not created in Redpanda Admin Panel/CLI or no events with this topic is created yet: {}", topic_str);
                    } else {
                        warn!("Error receiving event from topic {}: {:?}", topic_str, e);
                    }
                }
            }
        }
    }

}

