use axum::{
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "frontend/dist/"]
struct FrontEnd;

pub async fn static_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match FrontEnd::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(axum::body::Body::from(content.data))
                .unwrap()
        }
        None => {
            // SPA fallback: serve index.html for unknown routes
            match FrontEnd::get("index.html") {
                Some(content) => Html(content.data).into_response(),
                None => StatusCode::NOT_FOUND.into_response(),
            }
        }
    }
}
