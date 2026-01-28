# Routes & Endpoints

All routes are assembled in `src/startup.rs` inside `Application::build_app_router()`. API endpoints live under a nested sub-router at `/api/v1`, while web pages and static assets are mounted at the root.

## Route Table

| Method | Path | Handler | Module | Response Type |
|--------|------|---------|--------|---------------|
| GET | `/` | `home_page` | `routes/page.rs` | HTML (Tera template) |
| GET | `/robots.txt` | `robots_txt` | `routes/robots.rs` | `text/plain` |
| GET | `/sitemap.xml` | `sitemap_xml` | `routes/sitemap.rs` | `text/xml` |
| GET | `/static/datastar.js` | `DatastarScript` | Rama built-in | JavaScript |
| GET | `/static/*` | `ServeDir` | Rama built-in | Static files with cache control |
| GET | `/api/v1/health_check` | `health_check` | `routes/health_check.rs` | JSON |
| GET | `/api/v1/update` | `update_message` | `routes/update.rs` | HTML fragment (Datastar HPPA) |
| GET | `/api/v1/reset` | `reset_message` | `routes/update.rs` | HTML fragment (Datastar HPPA) |
| * | `*` (catch-all) | `not_found` | `routes/page.rs` | HTML 404 (Tera template) |

## Web Pages

### Home (`GET /`)

Renders `templates/index.html` with the content of `content/pages/index.md` converted to HTML at request time via `markdown_to_html()`. The template extends `base.html` and includes Datastar buttons for demonstrating dynamic updates.

### 404 Not Found (`*`)

Any path that does not match a registered route falls through to `not_found`, which renders `templates/404.html` and returns a `404 NOT FOUND` status code.

## SEO Endpoints

### Robots (`GET /robots.txt`)

Serves the contents of `static/robots.txt` with a `text/plain` content type. The file is embedded at compile time via `include_str!()`.

### Sitemap (`GET /sitemap.xml`)

Dynamically generates a sitemap XML document. Static page entries are defined in a vector within the handler; adding new pages to the site means adding an entry to that list.

## API Endpoints

### Health Check (`GET /api/v1/health_check`)

Returns a `200 OK` JSON response with an empty data payload. Useful for load balancer probes and deployment health checks.

```json
{
  "success": true,
  "message": "ok",
  "status": 200,
  "time": "2025-01-15T12:00:00Z",
  "data": null
}
```

### Update Message (`GET /api/v1/update`)

Returns an HTML fragment targeting `#test` and `#reset` divs. This is a Datastar HPPA (HTML Partial Push Action) response — the browser merges the fragment into the DOM without a full page reload.

### Reset Message (`GET /api/v1/reset`)

Returns empty HTML fragments for the same target divs, effectively clearing the content inserted by the update endpoint.

## Static Assets

All files under the `static/` directory are served by Rama's `ServeDir`. A `SetResponseHeaderLayer` adds `Cache-Control: public, max-age=604800` to every response that does not already include the header, giving browsers a one-week cache window.

The Datastar library script is served via Rama's built-in `DatastarScript` handler at `/static/datastar.js`, which takes precedence over the directory fallback.
