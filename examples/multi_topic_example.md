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
# Send to bounded-context-1 topic
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "123", "timestamp": "2024-01-01T00:00:00Z", "event_type": "BoundedContext1", "payload": {"type": "BoundedContext1Event", "action": "Created", "message": "User login successful"}}'

# Send to bounded-context-2 topic
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "124", "timestamp": "2024-01-01T00:00:01Z", "event_type": "BoundedContext2", "payload": {"type": "BoundedContext2Event", "action": "Created", "message": "Application startup complete"}}'
```

### Check server health

```bash
curl http://localhost:{{server_port}}/health
```

Expected response:
```json
{
  "message": "Server is healthy",
  "data": null
}
```

## Example 2: Real-World Scenario

### Simulate a microservices architecture

```bash
# Service A: User Service - sends user events
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "125", "timestamp": "2024-01-01T00:00:02Z", "event_type": "BoundedContext1", "payload": {"type": "BoundedContext1Event", "action": "Created", "message": "User created: john.doe@example.com"}}'

# Service B: Notification Service - sends notifications
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "126", "timestamp": "2024-01-01T00:00:03Z", "event_type": "BoundedContext2", "payload": {"type": "BoundedContext2Event", "action": "Created", "message": "Welcome email sent to john.doe@example.com"}}'

# Service C: Analytics Service - sends metrics
curl -X POST http://localhost:{{server_port}}/send \
  -H "Content-Type: application/json" \
  -d '{"id": "127", "timestamp": "2024-01-01T00:00:04Z", "event_type": "BoundedContext2", "payload": {"type": "BoundedContext2Event", "action": "Created", "message": "User registration metric: +1"}}'
```

## Example 3: Continuous Message Consumption

The application automatically starts continuous consumers for the following topics:
- `bounded-context-1`
- `bounded-context-2`

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
  "event_type": "BoundedContext1|BoundedContext2",
  "payload": {
    "type": "BoundedContext1Event|BoundedContext2Event",
    "action": "Created|Updated",
    "message": "Event description"
  }
}
```

## Event Actions

### BoundedContext1 Actions
- `Created`: User or business entity creation events
- `Updated`: User or business entity update events

### BoundedContext2 Actions
- `Created`: Application or system creation events
- `Updated`: Application or system update events

## Follow-up Events

The application demonstrates event chaining where processing one event can trigger another:
- When a `BoundedContext1` event is processed, it may trigger a `BoundedContext2` event
- This shows how different bounded contexts can interact through events
