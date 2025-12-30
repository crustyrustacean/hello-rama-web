# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build (optimized with LTO)

# Run
cargo run                      # Local development (127.0.0.1:8000)
APP_ENVIRONMENT=production cargo run --release  # Production mode

# Test
cargo test                     # Run all tests
cargo test --test api          # Run integration tests only
cargo test health_check_works  # Run a single test
TEST_LOG=1 cargo test -- --nocapture  # Tests with logging output

# Lint and format
cargo fmt
cargo clippy
```

## Architecture Overview

This is a web application using the [Rama](https://ramaproxy.org) HTTP framework with Tera templates and markdown content rendering.

### Request Flow

1. **Entry** (`src/bin/main.rs`): Initializes tracing, loads config, builds and runs the application
2. **Startup** (`src/startup.rs`): `Application::build()` compiles templates into `AppState`, configures routes, binds TCP listener
3. **Routes** (`src/routes/`): Handlers receive `AppState` with compiled Tera templates
4. **Response** (`src/response.rs`): `ApiResponse<T>` wrapper provides consistent JSON structure

### Key Design Patterns

- **Static template compilation**: Templates are compiled once via `OnceLock` in `src/templates.rs` and stored in `AppState`
- **Compile-time content embedding**: Markdown files from `content/` are embedded via `include_str!()` in route handlers
- **Layered configuration**: `configuration/base.yaml` → `configuration/{environment}.yaml` → `APP_*` env vars
- **Error conversion**: `ApiError` enum in `src/errors.rs` implements `IntoResponse` for automatic HTTP status mapping

### Module Structure

- `startup.rs` - Application builder and router configuration
- `state.rs` - AppState holding compiled Tera templates
- `configuration.rs` - Settings loading with config crate
- `errors.rs` - ApiError enum with HTTP status mappings
- `response.rs` - ApiResponse wrapper type
- `routes/` - HTTP handlers (health_check, render_page with home/404)
- `markdown.rs` - pulldown-cmark conversion with extended features
- `templates.rs` - Static Tera template compilation

### Configuration

Environment variables use `APP_` prefix with double underscore for nesting:
- `APP_ENVIRONMENT` - "local" or "production"
- `APP_APPLICATION__HOST` - Server host
- `APP_APPLICATION__PORT` - Server port
- `APP_APPLICATION__SHUTDOWN_TIMEOUT` - Graceful shutdown seconds

### Testing

Integration tests in `tests/api/` use `helpers::send_request()` to test routes through the Rama service layer without network I/O. Unit tests for markdown parsing are in `src/markdown.rs`.
