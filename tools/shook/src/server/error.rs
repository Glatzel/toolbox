use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Microsandbox(#[from] microsandbox::MicrosandboxError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Validator(#[from] validator::ValidationErrors),

    #[error("Missing header: {0}")]
    MissingHeader(String),
    #[error("Request signatures didn't match!")]
    RequestSignaturesMismatch,
    #[error("{0}")]
    Parse(String),
}
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Microsandbox(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Error::SerdeJson(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Error::Validator(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),

            Error::MissingHeader(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Error::RequestSignaturesMismatch => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Error::Parse(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
        }
    }
}
