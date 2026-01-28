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

# Documentation (mdbook)
mdbook build              # Build static HTML into ./book/
mdbook serve              # Build and serve at http://localhost:3000 with live-reload
mdbook watch              # Watch for changes and rebuild automatically
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

- `startup.rs` - Application builder, router configuration, and sub-router setup
- `state.rs` - AppState holding compiled Tera templates
- `configuration.rs` - Settings loading with config crate
- `errors.rs` - ApiError enum with HTTP status mappings
- `response.rs` - ApiResponse wrapper type
- `telemetry.rs` - Tracing subscriber setup and per-request span creation
- `markdown.rs` - pulldown-cmark conversion with extended features
- `templates.rs` - Static Tera template compilation via OnceLock
- `routes/` - HTTP handlers:
  - `health_check.rs` - `GET /api/v1/health_check`
  - `page.rs` - `GET /` (home) and catch-all 404
  - `robots.rs` - `GET /robots.txt` (served from static/robots.txt)
  - `sitemap.rs` - `GET /sitemap.xml` (dynamically generated)
  - `update.rs` - `GET /api/v1/update` and `GET /api/v1/reset` (Datastar HPPA fragments)

### Route Map

Routes are organized using Rama's sub-router pattern in `startup.rs`:

| Method | Path | Handler | Notes |
|--------|------|---------|-------|
| GET | `/` | `home_page` | Renders index.md via Tera |
| GET | `/robots.txt` | `robots_txt` | Serves static/robots.txt |
| GET | `/sitemap.xml` | `sitemap_xml` | Dynamically generated XML |
| GET | `/static/datastar.js` | `DatastarScript` | Rama built-in Datastar serve |
| GET | `/static/*` | `ServeDir` | Static assets with 1-week cache control |
| GET | `/api/v1/health_check` | `health_check` | Returns 200 OK JSON |
| GET | `/api/v1/update` | `update_message` | Datastar HPPA fragment |
| GET | `/api/v1/reset` | `reset_message` | Datastar HPPA fragment |
| * | `*` (catch-all) | `not_found` | 404 HTML page |

### Configuration

Environment variables use `APP_` prefix with double underscore for nesting:
- `APP_ENVIRONMENT` - "local" or "production"
- `APP_APPLICATION__HOST` - Server host
- `APP_APPLICATION__PORT` - Server port
- `APP_APPLICATION__SHUTDOWN_TIMEOUT` - Graceful shutdown seconds

### Testing

Integration tests in `tests/api/` use `helpers::send_request()` to test routes through the Rama service layer without network I/O. Unit tests for markdown parsing are in `src/markdown.rs`.

### Documentation

The `docs/` directory contains an [mdbook](https://github.com/rust-lang/mdBook) site with detailed coverage of the architecture, routes, configuration, and testing. Build with `mdbook build` or serve locally with `mdbook serve`.
