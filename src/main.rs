use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

mod types;
use types::{User, CreateUser};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let addr = "0.0.0.0:3000";
    tracing::info!("listening on {:?}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user: User = payload.into();

    (StatusCode::CREATED, Json(user))
}

