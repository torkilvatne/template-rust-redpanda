# Rust Redpanda Template - Architecture Overview

This document describes the modular architecture of the Rust Redpanda Template application.

## Directory Structure

```
src/
├── lib.rs                 # Library entry point and module declarations
├── main.rs               # Binary entry point
├── shared/               # Shared data structures and types
│   ├── mod.rs           # Shared module entry point
│   └── models.rs        # Event models, types, and handler registry
├── api/                  # HTTP API layer
│   ├── mod.rs           # API module entry point
│   ├── handlers.rs      # HTTP request handlers
│   └── routes.rs        # Route definitions and router setup
├── config/              # Configuration management
│   └── mod.rs           # Application configuration
├── events/              # Event processing layer
│   ├── mod.rs           # Events module entry point
│   └── processor.rs     # Event processor and consumer management
├── domains/             # Domain-driven design with bounded contexts
│   ├── mod.rs           # Domains module entry point
│   ├── bounded_context_1/ # First bounded context
│   │   ├── mod.rs       # Context builder and exports
│   │   ├── models.rs    # Context-specific models
│   │   ├── topic_handler.rs # Event processing logic
│   │   └── api.rs       # Context-specific API
│   └── bounded_context_2/ # Second bounded context
│       ├── mod.rs       # Context builder and exports
│       ├── models.rs    # Context-specific models
│       ├── topic_handler.rs # Event processing logic
│       └── api.rs       # Context-specific API
└── infrastructure/      # External service integrations
    ├── mod.rs           # Infrastructure module entry point
    └── redpanda.rs      # Redpanda/Kafka client implementation
```

## Module Responsibilities

### `lib.rs`
- Main library entry point
- Module declarations and re-exports
- Provides clean public API for the crate

### `main.rs`
- Binary entry point
- Application startup and configuration
- Server initialization and event processor setup

### `shared/` - Shared Data Structures
- **`models.rs`**: Event structures, types, and handler registry
- Common data models used across the application
- Event handler registration and management

### `api/` - HTTP API Layer
- **`handlers.rs`**: HTTP request handlers for health checks and event sending
- **`routes.rs`**: Route definitions and Axum router configuration

### `config/` - Configuration Management
- **`mod.rs`**: Environment-based configuration loading
- Server and Redpanda connection settings

### `events/` - Event Processing Layer
- **`processor.rs`**: Event processor that manages consumers and handlers
- Coordinates event consumption across multiple topics

### `domains/` - Domain-Driven Design
- **`bounded_context_1/`**: First bounded context with user and business logic
  - **`topic_handler.rs`**: Event processing logic for bounded context 1
  - **`models.rs`**: Context-specific data models
  - **`api.rs`**: Context-specific API endpoints
- **`bounded_context_2/`**: Second bounded context with application and system logic
  - **`topic_handler.rs`**: Event processing logic for bounded context 2
  - **`models.rs`**: Context-specific data models
  - **`api.rs`**: Context-specific API endpoints

### `infrastructure/` - External Service Integrations
- **`redpanda.rs`**: Redpanda/Kafka client implementation
- Message production and consumption logic

## Benefits of This Structure

1. **Separation of Concerns**: Each module has a clear, single responsibility
2. **Domain-Driven Design**: Clear boundaries between different business domains
3. **Testability**: Modules can be tested independently
4. **Maintainability**: Changes to one area don't affect others
5. **Scalability**: Easy to add new features without modifying existing code
6. **Reusability**: Modules can be reused in different contexts

## Adding New Features

### Adding a New Bounded Context
1. Create a new directory in `src/domains/` (e.g., `bounded_context_3/`)
2. Add the context to `src/domains/mod.rs`
3. Create the context builder, models, topic handler, and API
4. Add the new event type to `src/shared/models.rs`
5. Register the handler in `src/events/processor.rs`

### Adding a New API Endpoint
1. Add the handler function to `src/api/handlers.rs`
2. Add the route to `src/api/routes.rs`

### Adding a New External Service
1. Create a new module in `src/infrastructure/`
2. Implement the service client
3. Add it to `src/infrastructure/mod.rs`

## Template Variables

This template uses the following variables that will be replaced during generation:

- `{{crate_name}}`: The name of the generated package
- `{{server_port}}`: The default server port



