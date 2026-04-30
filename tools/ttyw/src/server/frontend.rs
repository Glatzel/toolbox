use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "frontend/dist/"]
struct FrontEnd;

pub async fn static_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    clerk::trace!(path, "Static file requested");

    match FrontEnd::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            clerk::trace!(path, mime = mime.as_ref(), "Serving embedded asset");
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(axum::body::Body::from(content.data.into_owned()))
                .unwrap()
        }
        None => match FrontEnd::get("index.html") {
            Some(content) => {
                clerk::debug!(path, "Asset not found, falling back to index.html");
                Html(String::from_utf8(content.data.into_owned()).unwrap_or_default())
                    .into_response()
            }
            None => {
                clerk::warn!(
                    path,
                    "Asset not found and index.html missing — returning 404"
                );
                StatusCode::NOT_FOUND.into_response()
            }
        },
    }
}
