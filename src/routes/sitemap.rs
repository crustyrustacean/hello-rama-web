// src/routes/sitemap.rs

// dependencies
use crate::errors::ApiError;
use crate::state::AppState;
use rama::http::header::CONTENT_TYPE;
use rama::http::response::Response;
use rama::http::service::web::extract::State;

pub async fn sitemap_xml(State(_state): State<AppState>) -> Result<Response, ApiError> {
    let base_url = "https://hello-rama-web";

    // Static pages
    let static_pages = vec![
        ("", "1.0"), // home
    ];

    // Build XML
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    // Add static pages
    for (path, priority) in static_pages {
        xml.push_str(&format!(
            "<url><loc>{}{}</loc><priority>{}</priority></url>",
            base_url, path, priority
        ));
    }

    xml.push_str("</urlset>");

    Response::builder()
        .header(CONTENT_TYPE, "text/xml")
        .body(xml.into())
        .map_err(|e| ApiError::InternalServerError(e.to_string()))
}
