set shell := ["bash", "-cu"]

default:
    just --list

# === Workspace Management ===
# Build all crates in the workspace
build:
    cargo build

# Run docker and the service
run:
    docker-compose up -d && cargo run

# Stop docker
stop:
    docker-compose down

# Run tests for all crates
test:
    cargo test --workspace

# Check all crates
check:
    cargo check --workspace

# Format all code
fmt:
    cargo fmt --all

# Clean build artifacts
clean:
    cargo clean