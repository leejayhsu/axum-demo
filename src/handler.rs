use crate::error::{AppError, BadRequest};
use crate::model;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

// basic handler that responds with a static string
pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"msg": "ok"})))
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<model::CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    // insert your application logic here
    let user = model::User {
        id: 1337,
        username: payload.username,
    };

    // just an example of using using error handling, get rid of this eventually.
    if user.username == "leej" {
        return Err(AppError::ClientError(BadRequest::NameTooShort));
    }
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(user)))
}
