// src/routes/static_files.rs

// dependencies
use crate::assets::Asset;
use rama::http::Body;
use rama::http::Response;
use rama::http::StatusCode;
use rama::http::service::web::response::IntoResponse;

macro_rules! static_file_handler {
    ($fn_name:ident, $filename:expr, $content_type:expr) => {
        pub async fn $fn_name() -> impl IntoResponse {
            let asset = Asset::get($filename).unwrap();
            let contents = std::str::from_utf8(asset.data.as_ref())
                .unwrap()
                .to_string();

            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", $content_type)
                .body(contents)
                .unwrap()
        }
    };

    ($fn_name:ident, $filename:expr, $content_type:expr, binary) => {
        pub async fn $fn_name() -> impl IntoResponse {
            let asset = Asset::get($filename).unwrap();
            let contents = asset.data.as_ref().to_vec();

            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", $content_type)
                .body(Body::from(contents))
                .unwrap()
        }
    };
}

// endpoint which returns 200 OK and the css styles
static_file_handler!(get_css_file, "styles.css", "text/css; charset=utf-8");

// endpoint which returns 200 OK and the JavaScript scripts
static_file_handler!(get_scripts_file, "scripts.js", "text/javascript");

// endpoint which returns 200 OK and the favicon file
static_file_handler!(get_image_file, "favicon.png", "image/png", binary);
