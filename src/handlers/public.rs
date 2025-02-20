use askama::Template;
use axum::response::{Html, IntoResponse, Response};

use crate::models::templates::{
    CreateTemplate, HomeTemplate, NotFoundTemplate, ServerErrorTemplate,
};

pub async fn home_handler() -> Response {
    let html_string = HomeTemplate {}.render().unwrap();
    Html(html_string).into_response()
}

pub async fn create_handler() -> Response {
    let html_string = CreateTemplate {}.render().unwrap();
    Html(html_string).into_response()
}

pub async fn not_found_handler() -> Response {
    let html_string = NotFoundTemplate {}.render().unwrap();
    Html(html_string).into_response()
}

pub async fn server_error_handler() -> Response {
    let html_string = ServerErrorTemplate {}.render().unwrap();
    Html(html_string).into_response()
}
