use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Our app's top level error type.
pub enum AppError {
    /// Something went wrong when calling the user repo.
    ClientError(BadRequest),
}

/// This makes it possible to use `?` to automatically convert a `UserRepoError`
/// into an `AppError`.
impl From<BadRequest> for AppError {
    fn from(inner: BadRequest) -> Self {
        AppError::ClientError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ClientError(BadRequest::NameTooShort) => {
                (StatusCode::BAD_REQUEST, "name too short")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

/// Errors that can happen when using the user repo.
#[derive(Debug)]
pub enum BadRequest {
    #[allow(dead_code)]
    NameTooShort,
}
