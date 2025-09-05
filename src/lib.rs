pub mod api;
pub mod config;
{% if use_redpanda %}
pub mod infrastructure;
{% endif %}
pub mod domains;
{% if use_redpanda %}
pub mod events;
{% endif %}
pub mod shared;

{% if use_redpanda %}
pub use infrastructure::*;
{% endif %}
pub use shared::*;
{% if use_redpanda %}
pub use events::processor::EventProcessor;
{% endif %}
