use axum::http::StatusCode;
use axum::response::IntoResponse;
use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
pub enum ShookServerError {
    #[diagnosis(transparent)]
    Microsandbox(#[diagnosis(from)] microsandbox::MicrosandboxError),
    #[diagnosis(transparent)]
    SerdeJson(#[diagnosis(from)] serde_json::Error),
    #[diagnosis(transparent)]
    Validator(#[diagnosis(from)] validator::ValidationErrors),

    #[diagnosis(description = "Missing header: {0}")]
    MissingHeader(String),
    #[diagnosis(description = "Request signatures didn't match!")]
    RequestSignaturesMismatch,
    #[diagnosis(description = "{0}")]
    Parse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_display_is_unchanged() {
        assert_eq!(
            ShookServerError::MissingHeader("X-Hub-Signature-256".into()).to_string(),
            "Missing header: X-Hub-Signature-256"
        );
        assert_eq!(
            ShookServerError::RequestSignaturesMismatch.to_string(),
            "Request signatures didn't match!"
        );
        assert_eq!(
            ShookServerError::Parse("invalid payload".into()).to_string(),
            "invalid payload"
        );
    }
}

impl IntoResponse for ShookServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Microsandbox(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Self::SerdeJson(_)
            | Self::Validator(_)
            | Self::MissingHeader(_)
            | Self::RequestSignaturesMismatch
            | Self::Parse(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
        }
    }
}
