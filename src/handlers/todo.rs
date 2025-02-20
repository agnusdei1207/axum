use askama::Template;
use axum::response::{Html, IntoResponse, Response};

use crate::models::templates::{CreateTodoTemplate, TodoTemplate};

async fn todo_handler() -> Response {
    let html_string = TodoTemplate {}.render().unwrap();
    Html(html_string).into_response()
}

async fn create_todo_handler() -> Response {
    let html_string = CreateTodoTemplate {}.render().unwrap();
    Html(html_string).into_response()
}
