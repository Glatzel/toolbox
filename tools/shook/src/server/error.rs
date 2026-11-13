use std::error::Error;
use std::fmt;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use mischief::IDiagnosis;

#[derive(Debug, IDiagnosis)]
pub enum ShookServerError {
    #[diagnosis(description = "microsandbox error")]
    Microsandbox(#[diagnosis(from)] microsandbox::MicrosandboxError),
    #[diagnosis(description = "JSON serialization error")]
    SerdeJson(#[diagnosis(from)] serde_json::Error),
    #[diagnosis(description = "request validation failed")]
    Validator(#[diagnosis(from)] validator::ValidationErrors),

    #[diagnosis(description = "missing request header")]
    MissingHeader(String),
    #[diagnosis(description = "request signatures did not match")]
    RequestSignaturesMismatch,
    #[diagnosis(description = "request parse error")]
    Parse(String),
}

impl fmt::Display for ShookServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Microsandbox(source) => fmt::Display::fmt(source, f),
            Self::SerdeJson(source) => fmt::Display::fmt(source, f),
            Self::Validator(source) => fmt::Display::fmt(source, f),
            Self::MissingHeader(header) => write!(f, "Missing header: {header}"),
            Self::RequestSignaturesMismatch => f.write_str("Request signatures didn't match!"),
            Self::Parse(message) => f.write_str(message),
        }
    }
}

impl Error for ShookServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Microsandbox(source) => Error::source(source),
            Self::SerdeJson(source) => Error::source(source),
            Self::Validator(source) => Error::source(source),
            _ => None,
        }
    }
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
