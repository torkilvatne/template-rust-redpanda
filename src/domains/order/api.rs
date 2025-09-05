use crate::shared::ApiResponse;
{% if use_redpanda %}
use crate::shared::Event;
use crate::infrastructure::RedpandaClient;
{% endif %}
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
{% if use_redpanda %}
use tracing::{debug, info, error};
{% endif %}
{% if use_redpanda == false %}
use tracing::{debug, info};
{% endif %}

pub async fn checkout_order(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    debug!("Checkout order endpoint called with payload: {:?}", payload);
    info!("Validating order payload and other things...");

    {% if use_redpanda %}
    // When Redpanda is enabled, try to deserialize and send the event
    let event: Event = match serde_json::from_value(payload) {
        Ok(e) => e,
        Err(e) => {
            let response = ApiResponse {
                message: format!("Invalid event payload: {}", e),
                data: None,
            };
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    let redpanda_client = RedpandaClient::new();
    match redpanda_client.send_event(&event).await {
        Ok(_) => {
            let topic_str = event.event_type.as_str();
            let response = ApiResponse {
                message: "Message sent successfully".to_string(),
                data: Some(serde_json::json!({ "topic": topic_str })),
            };
            debug!("Checkout order response: {:?}", response);
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            error!("Failed to send message: {}", e);
            let response = ApiResponse {
                message: format!("Failed to send message: {}", e),
                data: None,
            };
            debug!("Send message error response: {:?}", response);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
    {% else %}
    // When disabled, simply acknowledge
    let response = ApiResponse {
        message: "Message queue is disabled in this template".to_string(),
        data: None,
    };
    (StatusCode::OK, Json(response))
    {% endif %}
}
