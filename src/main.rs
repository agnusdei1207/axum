use askama::Template;
use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home))
        .route("/create", get(create))
        .route("/not-found", get(not_found))
        .route("/server-error", get(server_error))
        .route("/sign-up", get(sign_up))
        .route("/todo", get(todo))
        .nest_service("/static", server_dir);

    axum::serve(listener, app).await.unwrap();
}
