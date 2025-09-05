# {{crate_name}}

{{project_description}}

A Rust application with Redpanda integration featuring multi-topic event streaming and continuous message consumption.

## Features

- **Multi-Topic Support**: Send messages to different topics for event organization
- **Domain-Driven Design**: Separate bounded contexts with dedicated event handlers
- **Continuous Consumption**: Background consumers that process messages as they arrive
- **Event-Driven Architecture**: Separate topics for different types of events
- **Scalable Design**: Each topic can be scaled independently

## Services

- **Rust Server**: HTTP API on port {{server_port}}
- **Redpanda**: Kafka-compatible message broker on port 9093
- **Redpanda Console**: Web UI for managing topics and messages on port 8080

## Quick Start

1. **Start all services:**
   ```bash
   just run
   ```
   or
   ```bash
   docker compose up -d && cargo run
   ```

2. **Check service status:**
   ```bash
   docker compose ps
   ```

3. **View logs:**
   ```bash
   docker compose logs -f
   ```

4. **Run tests:**
   ```bash
   ./test.sh
   ```

## API Endpoints

The Rust server provides the following endpoints:

- `POST /send` - Send an event to a specific topic
- `POST /order` - Checkout order (sends an event)

### Example Usage

**Send an event to a specific topic:**
```bash
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{
    "id": "123",
    "timestamp": "2024-01-01T00:00:00Z",
    "event_type": "order",
    "payload": {"type": "OrderEvent", "action": "Created", "message": "Order created"}
  }'
```

**Checkout order (domain-specific endpoint):**
```bash
curl -X POST http://localhost:{{server_port}}/order \
  -H "Content-Type: application/json" \
  -d '{
    "id": "456",
    "timestamp": "2024-01-01T00:00:00Z",
    "event_type": "order",
    "payload": {"type": "OrderEvent", "action": "Created", "message": "Checkout initiated"}
  }'
```

## Default Topics

The application automatically starts continuous consumers for these topics:

- `order`: Order bounded context
- `logistics`: Logistics bounded context

## Access Points

- **Rust Server API**: http://localhost:{{server_port}}
- **Redpanda Console**: http://localhost:8080
- **Redpanda Kafka API**: localhost:9093

## Stopping Services
**Stop all services**

```bash
just stop
```
or
```bash
docker compose down
```

**Stop and remove volumes (WARNING: deletes all data)**
```
docker compose down -v
```

## Project Structure

The Rust server is organized into modular components:

- `src/main.rs` - Main application entry point, server setup, and event processor initialization
- `src/shared/models.rs` - Data structures and types (Event, EventType, EventPayload, EventHandlerRegistry)
- `src/infrastructure/redpanda.rs` - Redpanda/Kafka client functionality with continuous consumers and topic-specific handlers
- `src/api/handlers.rs` - HTTP endpoint handlers
- `src/events/processor.rs` - Event processor that manages consumers and handlers
- `src/domains/` - Domain-driven design with bounded contexts and topic handlers
- `examples/` - Documentation and usage examples

### Domain Structure

The application follows a domain-driven design with bounded contexts:

- **Order**: Handles order-related events (Created/Updated)
- **Logistics**: Handles logistics-related events (Created/Updated)

Each bounded context has its own topic handler that processes events and can trigger follow-up events.

## Architecture

This application implements a multi-topic event-driven architecture:

1. **Producer**: Sends events to specific topics via the `/send` endpoint
2. **Consumer**: Continuously consumes events from multiple topics in the background
3. **Topic Separation**: Different types of events are sent to different topics for better organization
4. **Domain Handlers**: Each bounded context has dedicated event handlers for processing
5. **Scalability**: Each topic can be scaled independently

## Development

To modify the Rust server:

1. Edit the appropriate module based on what you're changing:
   - For new endpoints: `src/api/handlers.rs`
   - For data structures: `src/shared/models.rs`
   - For Redpanda logic: `src/infrastructure/redpanda.rs`
   - For event processing: `src/events/processor.rs`
   - For domain logic: `src/domains/`
   - For server configuration: `src/main.rs`
2. Rebuild the container:
   ```bash
   docker compose build rust-server
   docker compose up -d rust-server
   ```

## Examples

See `examples/multi_topic_example.md` for detailed usage examples and real-world scenarios.
