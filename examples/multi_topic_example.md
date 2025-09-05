# Multi-Topic Example Usage

This document demonstrates how to use the multi-topic functionality in the Rust Redpanda application.

## Prerequisites

1. Start the services:
   ```bash
   docker compose up -d
   ```

2. Wait for services to be ready (check with `docker compose ps`)

## Example 1: Basic Multi-Topic Operations

### Send events to different topics

```bash
# Send to order topic
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "123", "timestamp": "2024-01-01T00:00:00Z", "event_type": "order", "payload": {"type": "OrderEvent", "action": "Created", "message": "Order created"}}'

# Send to logistics topic
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "124", "timestamp": "2024-01-01T00:00:01Z", "event_type": "logistics", "payload": {"type": "LogisticsEvent", "action": "Created", "message": "Shipment created"}}'
```

### Use domain-specific endpoint

```bash
curl -X POST http://localhost:{{server_port}}/order \
  -H "Content-Type: application/json" \
  -d '{"id": "200", "timestamp": "2024-01-01T00:00:05Z", "event_type": "order", "payload": {"type": "OrderEvent", "action": "Created", "message": "Checkout initiated"}}'
```

## Example 2: Real-World Scenario

### Simulate a microservices architecture

```bash
# Service A: Order Service - sends order events
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "125", "timestamp": "2024-01-01T00:00:02Z", "event_type": "order", "payload": {"type": "OrderEvent", "action": "Created", "message": "Order #1001 created"}}'

# Service B: Logistics Service - sends logistics events
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "126", "timestamp": "2024-01-01T00:00:03Z", "event_type": "logistics", "payload": {"type": "LogisticsEvent", "action": "Created", "message": "Shipment created for order #1001"}}'

# Service C: Analytics Service - sends metrics
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "127", "timestamp": "2024-01-01T00:00:04Z", "event_type": "logistics", "payload": {"type": "LogisticsEvent", "action": "Created", "message": "Order #1001 metric: shipped"}}'
```

## Example 3: Continuous Message Consumption

The application automatically starts continuous consumers for the following topics:
- `order`
- `logistics`

Events sent to these topics will be consumed continuously in the background. Check the server logs to see consumed events:

```bash
docker compose logs -f rust-server
```

## Example 4: Using the Test Script

Run the comprehensive test script:

```bash
./test.sh
```

This script will:
1. Check server health
2. Send events to different topics
3. Demonstrate the API endpoints

## Event Structure

All events follow this structure:

```json
{
  "id": "unique-event-id",
  "timestamp": "ISO-8601-timestamp",
  "event_type": "order|logistics",
  "payload": {
    "type": "OrderEvent|LogisticsEvent",
    "action": "Created|Updated",
    "message": "Event description"
  }
}
```

## Event Actions

### Order Actions
- `Created`: Order creation events
- `Updated`: Order update events

### Logistics Actions
- `Created`: Logistics creation events
- `Updated`: Logistics update events

## Follow-up Events

The application demonstrates event chaining where processing one event can trigger another:
- When an `Order` event is processed, it may trigger a `Logistics` event
- This shows how different domains can interact through events
