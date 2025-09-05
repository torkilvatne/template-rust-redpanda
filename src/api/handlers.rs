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
use tracing::{debug, error};
{% endif %}
{% if use_redpanda == false %}
use tracing::{debug};
{% endif %}

pub async fn send_message(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    debug!("Send message endpoint called with payload: {:?}", payload);

    {% if use_redpanda %}
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
            debug!("Send message response: {:?}", response);
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
    let response = ApiResponse {
        message: "Message queue is disabled in this template".to_string(),
        data: None,
    };
    (StatusCode::OK, Json(response))
    {% endif %}
}