# Rust Redpanda Template

This is a `cargo-generate` template for creating Rust applications with Redpanda integration featuring multi-topic event streaming and continuous message consumption.

## Usage

To use this template, run:

```bash
cargo generate --init --path /path/to/templates/rust-redpanda-template
```

Or if you have this template in a git repository:

```bash
cargo generate --git https://github.com/your-username/rust-redpanda-template.git
```

## Template Variables

When you run the template, you'll be prompted for the following variables:

- **crate_name**: The name of your Rust project (default: `my-redpanda-app`)
- **server_port**: The port your server will run on (default: `3000`)

## What's Included

This template provides:

- A complete Rust web server using Axum
- Redpanda/Kafka integration with rdkafka
- Multi-topic event streaming support
- Domain-driven design with bounded contexts
- Continuous message consumption in the background
- Docker Compose setup with Redpanda and Redpanda Console
- Health check and event sending endpoints
- Comprehensive logging with tracing
- CORS support
- Ready-to-use test script
- Detailed examples and documentation

## Generated Project Structure

```
your-project/
├── Cargo.toml          # Rust dependencies and project config
├── src/
│   ├── lib.rs          # Library entry point and module declarations
│   ├── main.rs         # Main application entry point
│   ├── shared/         # Shared data structures and types
│   │   ├── mod.rs      # Shared module entry point
│   │   └── models.rs   # Event models and handler registry
│   ├── api/            # HTTP API layer
│   │   ├── mod.rs      # API module entry point
│   │   ├── handlers.rs # HTTP request handlers
│   │   └── routes.rs   # Route definitions and router setup
│   ├── config/         # Configuration management
│   │   └── mod.rs      # Application configuration
│   ├── events/         # Event processing layer
│   │   ├── mod.rs      # Events module entry point
│   │   └── processor.rs # Event processor and consumer management
│   ├── domains/        # Domain-driven design with bounded contexts
│   │   ├── mod.rs      # Domains module entry point
│   │   ├── bounded_context_1/ # First bounded context
│   │   │   ├── mod.rs  # Context builder and exports
│   │   │   ├── models.rs # Context-specific models
│   │   │   ├── topic_handler.rs # Event processing logic
│   │   │   └── api.rs  # Context-specific API
│   │   └── bounded_context_2/ # Second bounded context
│   │       ├── mod.rs  # Context builder and exports
│   │       ├── models.rs # Context-specific models
│   │       ├── topic_handler.rs # Event processing logic
│   │       └── api.rs  # Context-specific API
│   └── infrastructure/ # External service integrations
│       ├── mod.rs      # Infrastructure module entry point
│       └── redpanda.rs # Redpanda/Kafka client implementation
├── examples/
│   └── multi_topic_example.md  # Usage examples and documentation
├── docker-compose.yml  # Docker services (Redpanda + Console)
├── Dockerfile          # Rust application container
├── test.sh            # Test script for the API
├── ARCHITECTURE.md     # Architecture documentation
└── README.md          # Project documentation
```

## Quick Start

After generating your project:

1. Navigate to your new project directory
2. Start the services: `docker compose up -d`
3. Run the test script: `./test.sh`
4. Access the Redpanda Console at http://localhost:8080

## API Endpoints

- `GET /health` - Health check
- `POST /send` - Send an event to a specific topic

## Architecture

This template follows a clean, modular architecture:

- **API Layer** (`api/`): HTTP handlers and routing logic
- **Events Layer** (`events/`): Event processing and consumer management
- **Domains Layer** (`domains/`): Domain-driven design with bounded contexts
- **Configuration** (`config/`): Application settings and environment management
- **Infrastructure** (`infrastructure/`): External service integrations (Redpanda)
- **Shared** (`shared/`): Common data structures and types

See `ARCHITECTURE.md` for detailed documentation on the module structure and how to extend the application.

## Default Topics

The application automatically starts continuous consumers for these topics:

- `bounded-context-1`: User and business logic events
- `bounded-context-2`: Application and system events

## Features

- **Multi-Topic Support**: Send events to different topics for event organization
- **Domain-Driven Design**: Clean separation of bounded contexts with dedicated event handlers
- **Continuous Consumption**: Background consumers that process events as they arrive
- **Event-Driven Architecture**: Separate topics for different types of events
- **Scalable Design**: Each topic can be scaled independently
- **Modular Architecture**: Clean separation of concerns with dedicated modules for API, events, domains, configuration, and infrastructure
- **Testable Design**: Each module can be tested independently
- **Maintainable Code**: Clear module boundaries and responsibilities
