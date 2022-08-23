use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"msg": "ok"})))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };
    if user.username == "leej" {
        return Err(AppError::ClientError(BadRequest::NameTooLong));
    }
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(user)))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

/// Our app's top level error type.
enum AppError {
    /// Something went wrong when calling the user repo.
    ClientError(BadRequest),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ClientError(BadRequest::NameTooLong) => {
                (StatusCode::BAD_REQUEST, "Name too long")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

/// This makes it possible to use `?` to automatically convert a `UserRepoError`
/// into an `AppError`.
impl From<BadRequest> for AppError {
    fn from(inner: BadRequest) -> Self {
        AppError::ClientError(inner)
    }
}

/// Errors that can happen when using the user repo.
#[derive(Debug)]
enum BadRequest {
    #[allow(dead_code)]
    NameTooLong,
}