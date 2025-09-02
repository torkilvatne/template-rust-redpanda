use crate::api::handlers::{health_check, send_message};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/send", post(send_message))
}

