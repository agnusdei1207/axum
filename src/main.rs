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

async fn home() -> Response {
    let html_string = home_template {}.render().unwrap();

    Html(html_string).into_response()
}

async fn create() -> Response {
    let html_string = create_template {}.render().unwrap();
    Html(html_string).into_response()
}

async fn not_found() -> Response {
    let html_string = not_found_template {}.render().unwrap();
    Html(html_string).into_response()
}

async fn server_error() -> Response {
    let html_string = server_error_template {}.render().unwrap();
    Html(html_string).into_response()
}

async fn todo() -> Response {
    let html_string = todo_template {}.render().unwrap();
    Html(html_string).into_response()
}
