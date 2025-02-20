use askama::Template;
use axum::response::{Html, IntoResponse, Response};

use crate::models::templates::{CreateTodoTemplate, TodoTemplate};

pub async fn todo_handler() -> Response {
    let html_string = TodoTemplate {}.render().unwrap();
    Html(html_string).into_response()
}

pub async fn create_todo_handler() -> Response {
    let html_string = CreateTodoTemplate {}.render().unwrap();
    Html(html_string).into_response()
}
