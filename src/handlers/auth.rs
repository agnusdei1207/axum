use crate::models::templates::{LogInTemplate, SignUpTemplate};
use askama::Template;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

pub async fn sign_up_handler() -> Response {
    let html_string = SignUpTemplate {}.render().unwrap();
    Html(html_string).into_response()
}

pub async fn login_in_handler() -> Response {
    let html_string = LogInTemplate {}.render().unwrap();
    Html(html_string).into_response()
}
