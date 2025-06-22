mod routes;
mod services;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing

    // build our application with a router
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "hello, world"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1223,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}


#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}