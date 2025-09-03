use crate::shared::{Event, ApiResponse};
use crate::infrastructure::RedpandaClient;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{debug, info, error};

pub async fn checkout_order(Json(event): Json<Event>) -> impl IntoResponse {
    debug!("Checkout order endpoint called with payload: {:?}", event);

    info!("Validating order payload and other things...");
    
    
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
}
