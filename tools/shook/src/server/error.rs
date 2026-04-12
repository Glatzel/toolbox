use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum ShookServerError {
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
impl IntoResponse for ShookServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ShookServerError::Microsandbox(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            ShookServerError::SerdeJson(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            ShookServerError::Validator(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }

            ShookServerError::MissingHeader(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            ShookServerError::RequestSignaturesMismatch => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            ShookServerError::Parse(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
        }
    }
}
