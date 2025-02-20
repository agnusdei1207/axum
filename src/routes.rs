use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::handlers::{
    auth::sign_up_handler,
    public::{create_handler, home_handler, not_found_handler, server_error_handler},
    todo::todo_handler,
};

pub fn router() -> Router {
    let server_dir = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/create", get(create_handler))
        .route("/not-found", get(not_found_handler))
        .route("/server-error", get(server_error_handler))
        .route("/sign-up", get(sign_up_handler))
        .route("/todo", get(todo_handler))
        .nest_service("/static", server_dir);

    app
}
