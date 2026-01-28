# Hello Rama Web

A web application template built with [Rama](https://ramaproxy.org), [Tera](https://keats.github.io/tera), [Datastar](https://data-star.dev), and modern CSS.

## Features

- Server-rendered HTML pages via Tera templates
- Markdown content rendering with pulldown-cmark
- Datastar integration for dynamic UI updates (HPPA)
- Static asset serving with cache control headers
- SEO support via `/robots.txt` and `/sitemap.xml`
- JSON API with consistent response envelope
- Structured tracing and logging (Bunyan format)
- Graceful shutdown support
- Environment-based configuration (local / production)

## Getting Started

```bash
# Build and run in local development mode
cargo run

# Build and run in production mode
APP_ENVIRONMENT=production cargo run --release

# Run tests
cargo test

# Lint and format
cargo clippy
cargo fmt
```

The local server starts at `http://127.0.0.1:8000`.

## Project Structure

```
src/
├── bin/main.rs        # Entry point: tracing, config, start server
├── startup.rs         # Application builder and router setup
├── state.rs           # AppState (compiled templates)
├── configuration.rs   # Layered config loading
├── errors.rs          # ApiError enum with HTTP status mapping
├── response.rs        # ApiResponse<T> JSON wrapper
├── telemetry.rs       # Tracing subscriber and request spans
├── markdown.rs        # Markdown-to-HTML conversion
├── templates.rs       # Static Tera template compilation
└── routes/            # HTTP handlers
    ├── health_check.rs
    ├── page.rs        # Home page and 404
    ├── robots.rs
    ├── sitemap.rs
    └── update.rs      # Datastar HPPA endpoints
```

## Configuration

Settings are layered: `configuration/base.yaml` → `configuration/{environment}.yaml` → environment variables with the `APP_` prefix.

Key variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `APP_ENVIRONMENT` | `local` or `production` | `local` |
| `APP_APPLICATION__HOST` | Bind host | `127.0.0.1` (local) / `0.0.0.0` (prod) |
| `APP_APPLICATION__PORT` | Bind port | `8000` (local) / `8080` (prod) |
| `APP_APPLICATION__SHUTDOWN_TIMEOUT` | Graceful shutdown seconds | `10` |

## Documentation

Full project documentation is available as an [mdbook](https://github.com/rust-lang/mdBook) site in the [`docs/`](docs/) directory.

Install mdbook if you don't have it already:

```bash
cargo install mdbook
```

Then build or serve the docs locally:

```bash
mdbook serve   # Build and serve docs at http://localhost:3000
mdbook build   # Build static HTML into ./book/
```

The book covers:

- [Introduction](docs/introduction.md) — feature overview and getting started
- [Architecture](docs/architecture.md) — request flow, module map, and design patterns
- [Routes & Endpoints](docs/routes.md) — complete route table with per-endpoint descriptions
- [Configuration](docs/configuration.md) — layered settings, YAML files, and environment variables
- [Testing](docs/testing.md) — integration test structure, helpers, and unit test coverage
