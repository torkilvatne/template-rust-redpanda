use crate::{api::handlers::send_message, domains::order::api::checkout_order};
use axum::{
    routing::{post},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/order", post(checkout_order))
        .route("/send", post(send_message))
}

