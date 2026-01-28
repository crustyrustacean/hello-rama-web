# Introduction

Hello Rama Web is a web application template built with [Rama](https://ramaproxy.org), [Tera](https://keats.github.io/tera), [Datastar](https://data-star.dev), and modern CSS. It demonstrates how to compose a full-stack server-rendered web application in Rust without heavy framework abstractions.

## What This Template Provides

- **Server-rendered HTML** via Tera templates with compile-time markdown content embedding
- **Datastar integration** for lightweight dynamic UI updates using HPPA (HTML Partial Push Actions)
- **Static asset serving** with automatic cache control headers
- **SEO support** through `/robots.txt` and a dynamically generated `/sitemap.xml`
- **JSON API** with a consistent response envelope for programmatic consumers
- **Structured observability** via Bunyan-formatted tracing logs with per-request spans
- **Graceful shutdown** with a configurable timeout
- **Layered configuration** supporting local development and production deployment

## Getting Started

### Prerequisites

- Rust toolchain (stable)
- Cargo

### Build and Run

```bash
# Debug build and run (local development)
cargo run

# Release build and run (production)
APP_ENVIRONMENT=production cargo run --release
```

The local server binds to `http://127.0.0.1:8000` by default.

### Run Tests

```bash
cargo test                     # All tests
cargo test --test api          # Integration tests only
TEST_LOG=1 cargo test -- --nocapture  # With tracing output
```

### Lint and Format

```bash
cargo fmt
cargo clippy
```
