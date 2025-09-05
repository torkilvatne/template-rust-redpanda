#!/bin/bash

# Test script for multi-topic functionality
echo "Testing Multi-Topic Redpanda Application"
echo "========================================"

BASE_URL="http://localhost:{{server_port}}"

# Domain endpoint
echo -e "\n1. Order Endpoint:"
curl -s -X POST "$BASE_URL/order" \
  -H "Content-Type: application/json" \
  -d '{"id": "200", "timestamp": "2024-01-01T00:00:05Z", "event_type": "order", "payload": {"type": "OrderEvent", "action": "Created", "message": "Checkout initiated"}}' | jq '.'

# Send events to different topics
echo -e "\n2. Sending Events to Different Topics:"

echo -e "\n   Sending to 'order' topic:"
curl -s -X POST "$BASE_URL/send" \
  -H "Content-Type: application/json" \
  -d '{"id": "123", "timestamp": "2024-01-01T00:00:00Z", "event_type": "order", "payload": {"type": "OrderEvent", "action": "Created", "message": "Order created"}}' | jq '.'

echo -e "\n   Sending to 'logistics' topic:"
curl -s -X POST "$BASE_URL/send" \
  -H "Content-Type: application/json" \
  -d '{"id": "124", "timestamp": "2024-01-01T00:00:01Z", "event_type": "logistics", "payload": {"type": "LogisticsEvent", "action": "Created", "message": "Shipment created"}}' | jq '.'

echo -e "\n   Sending to 'order' topic (second event):"
curl -s -X POST "$BASE_URL/send" \
  -H "Content-Type: application/json" \
  -d '{"id": "125", "timestamp": "2024-01-01T00:00:02Z", "event_type": "order", "payload": {"type": "OrderEvent", "action": "Updated", "message": "Order updated"}}' | jq '.'

echo -e "\n   Sending to 'logistics' topic (second event):"
curl -s -X POST "$BASE_URL/send" \
  -H "Content-Type: application/json" \
  -d '{"id": "126", "timestamp": "2024-01-01T00:00:03Z", "event_type": "logistics", "payload": {"type": "LogisticsEvent", "action": "Updated", "message": "Shipment updated"}}' | jq '.'

# Wait a moment for events to be processed
echo -e "\n3. Waiting for events to be processed..."
sleep 2

# Note: Events are now consumed continuously in the background
echo -e "\n4. Continuous Event Consumption:"
echo -e "   Events are now being consumed continuously in the background."
echo -e "   Check the server logs to see consumed events."

echo -e "\nTest completed!"
echo ""
echo "🌐 Access points:"
echo "   - Rust Server API: http://localhost:{{server_port}}"
echo "   - Redpanda Console: http://localhost:8080"
echo "   - Redpanda Kafka: localhost:9093"
