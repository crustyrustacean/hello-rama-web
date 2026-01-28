# Configuration

Configuration is loaded by `get_configuration()` in `src/configuration.rs` using the [`config`](https://docs.rs/config) crate. Three sources are merged in order of increasing priority:

1. **Base** — `configuration/base.yaml` (shared defaults)
2. **Environment** — `configuration/{environment}.yaml` (environment-specific overrides)
3. **Environment variables** — `APP_*` prefixed variables (runtime overrides)

## Selecting the Environment

The active environment is determined by the `APP_ENVIRONMENT` variable. If unset, it defaults to `local`.

```bash
# Use local configuration (default)
cargo run

# Use production configuration
APP_ENVIRONMENT=production cargo run --release
```

Supported values: `local`, `production`.

## Configuration Files

### `configuration/base.yaml`

Shared defaults that apply regardless of environment.

```yaml
application:
  port: 8000
  template_dir: "templates"
  template_pattern: "**/*"
  shutdown_timeout: 10
```

### `configuration/local.yaml`

Overrides for local development.

```yaml
application:
  host: 127.0.0.1
```

### `configuration/production.yaml`

Overrides for production deployments. Binds to all interfaces and uses port 8080 for compatibility with hosting platforms such as Fly.io.

```yaml
application:
  host: 0.0.0.0
  port: 8080
```

## Environment Variables

Variables use the `APP_` prefix. Nested fields are separated by double underscores (`__`).

| Variable | Maps To | Description | Default |
|----------|---------|-------------|---------|
| `APP_ENVIRONMENT` | (environment selector) | Selects which YAML overlay to load | `local` |
| `APP_APPLICATION__HOST` | `application.host` | TCP bind address | `127.0.0.1` (local) / `0.0.0.0` (production) |
| `APP_APPLICATION__PORT` | `application.port` | TCP bind port | `8000` (local) / `8080` (production) |
| `APP_APPLICATION__TEMPLATE_DIR` | `application.template_dir` | Directory containing Tera templates | `templates` |
| `APP_APPLICATION__TEMPLATE_PATTERN` | `application.template_pattern` | Glob pattern for template discovery | `**/*` |
| `APP_APPLICATION__SHUTDOWN_TIMEOUT` | `application.shutdown_timeout` | Seconds to wait for in-flight requests during graceful shutdown | `10` |

## Settings Structs

```rust
pub struct Settings {
    pub application: ApplicationSettings,
}

pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub template_dir: String,
    pub template_pattern: String,
    pub shutdown_timeout: u64,
}
```

All numeric fields that may arrive as strings (e.g., from environment variables) use `serde_aux`'s `deserialize_number_from_string` attribute to handle the conversion transparently.
