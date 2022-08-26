use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
// basic handler that responds with a static string
pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"msg": "ok"})))
}
