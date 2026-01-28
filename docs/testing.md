# Testing

The project includes two categories of tests: integration tests for HTTP endpoints and unit tests for internal utilities.

## Integration Tests

Integration tests live in `tests/api/` and exercise routes through the full Rama service stack — router, middleware, and handlers — without binding a network socket.

### Test Modules

| File | What It Tests |
|------|---------------|
| `health_check.rs` | `GET /api/v1/health_check` returns `200 OK` |
| `index.rs` | `GET /` returns `200 OK` with `text/html` content type and a valid HTML document |
| `not_found.rs` | An unregistered path returns `404 NOT FOUND` with `text/html` content |
| `static_assets.rs` | CSS, JavaScript, and image assets are served with correct MIME types |

### Test Helper

`tests/api/helpers.rs` provides `send_request(uri)`, which:

1. Forces initialization of the global tracing subscriber (once, via `LazyLock`).
2. Loads the application configuration.
3. Compiles templates and constructs `AppState`.
4. Builds the router with the `TraceLayer` middleware.
5. Constructs an HTTP `Request` for the given URI and calls `service.serve()`.
6. Returns the `Response` for assertions.

This approach means tests run without network I/O, making them fast and deterministic.

### Tracing in Tests

By default, test tracing output is silenced. Set the `TEST_LOG` environment variable to enable it:

```bash
TEST_LOG=1 cargo test -- --nocapture
```

## Unit Tests

### Markdown Parsing (`src/markdown.rs`)

Unit tests cover the `markdown_to_html()` function:

- Basic headings, bold, and italic
- Fenced code blocks with language hints
- Links and images
- Ordered and unordered lists
- Tables
- Strikethrough (`~~text~~`)
- Task lists (`- [x]` / `- [ ]`)
- Inline code
- Empty input and plain text passthrough

## Running Tests

```bash
# All tests (integration + unit)
cargo test

# Integration tests only
cargo test --test api

# A single test by name
cargo test health_check_works

# With tracing output
TEST_LOG=1 cargo test -- --nocapture
```
