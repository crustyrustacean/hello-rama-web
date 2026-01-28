# Architecture

## Request Flow

```
Client
  │
  ▼
src/bin/main.rs          ← Entry point: initialize tracing, load config, start server
  │
  ▼
src/startup.rs           ← Application::build() compiles templates, wires the router,
  │                         binds the TCP listener, and runs with TraceLayer middleware
  │
  ▼
src/routes/*.rs          ← Handlers extract AppState, produce HTML or JSON responses
  │
  ▼
src/response.rs          ← ApiResponse<T> wraps JSON replies in a consistent envelope
src/errors.rs            ← ApiError converts to appropriate HTTP status via IntoResponse
```

## Module Overview

| Module | Responsibility |
|--------|----------------|
| `src/bin/main.rs` | Entry point. Initializes the Bunyan tracing subscriber, reads configuration, builds and runs the application. |
| `src/startup.rs` | `Application` struct. `build()` compiles templates, constructs `AppState`, assembles the router with sub-routers, and binds the TCP listener. `run()` starts the HTTP server with tracing middleware and graceful shutdown. |
| `src/state.rs` | `AppState` struct holding a `&'static Tera` reference to the compiled templates. Passed into every route handler via Rama's `State` extractor. |
| `src/configuration.rs` | `Settings` and `ApplicationSettings` structs. `get_configuration()` layers `base.yaml`, the environment-specific YAML, and `APP_*` environment variables. |
| `src/errors.rs` | `ApiError` enum covering common HTTP error statuses. Implements `IntoResponse` to return a JSON error body with the correct status code. Re-exports Rama's `BoxError`, `OpaqueError`, and `ErrorContext` for application-level error handling. |
| `src/response.rs` | `ApiResponse<T>` generic struct serialized as JSON with `success`, `message`, `status`, `time`, and `data` fields. Implements `IntoResponse`. |
| `src/telemetry.rs` | `get_subscriber()` builds a `tracing-subscriber` with Bunyan formatting. `init_subscriber()` sets it as the global default. `make_request_span()` creates a per-request span with method, URI, and a UUID. |
| `src/markdown.rs` | `markdown_to_html()` converts Markdown to HTML using pulldown-cmark with strikethrough, tables, footnotes, task lists, and smart punctuation enabled. |
| `src/templates.rs` | `compile_templates()` compiles Tera templates once into a `OnceLock<Tera>` and returns a `&'static` reference for the lifetime of the process. |
| `src/routes/` | Individual handler modules (see [Routes & Endpoints](./routes.md)). |

## Key Design Patterns

### Static Template Compilation

Templates are compiled exactly once at startup via `std::sync::OnceLock` in `src/templates.rs`. The resulting `&'static Tera` reference is stored in `AppState` and cloned into each request context at zero cost.

### Compile-Time Content Embedding

Page content lives in `content/pages/*.md` and is pulled into route handlers at compile time with `include_str!()`. This avoids runtime file I/O for static content while keeping the source files editable outside of `src/`.

### Layered Configuration

Configuration merges three sources in priority order:

1. `configuration/base.yaml` — shared defaults
2. `configuration/{environment}.yaml` — environment overrides (local or production)
3. `APP_*` environment variables — runtime overrides (double underscore `__` for nesting)

### Error Conversion via IntoResponse

`ApiError` variants map directly to HTTP status codes. Implementing Rama's `IntoResponse` trait means handlers can return `Result<T, ApiError>` and the framework automatically converts errors into properly-formatted JSON responses with the correct status.

### Sub-Router Organization

API routes live under a nested sub-router (`/api/v1/...`) while web pages and static assets are mounted at the root. This keeps the routing tree clean and makes it straightforward to version the API or add further sub-trees.

### Cache-Controlled Static Assets

The `ServeDir` for `/static` is wrapped in a `SetResponseHeaderLayer` that unconditionally sets `Cache-Control: public, max-age=604800` (one week) on all responses that don't already have the header.
