use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::Form;
use serde::Deserialize;

use crate::{course, custom::HtmlResponse, subject, view::render_template};

#[derive(Deserialize, Debug)]
pub struct CreateSubjectControllerModel {
    code: String,
    name: String,
    program: String,
    courses: Vec<String>,
}

pub async fn list_html() -> impl IntoResponse {
    match super::service::SERVICE.list_with_courses().await {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(subjects) => render_template("subject/list", subjects.into()).to_html_response(),
    }
}

pub async fn create_html() -> impl IntoResponse {
    match course::service::SERVICE.list_courses().await {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(courses) => render_template("subject/create", courses.into()).to_html_response(),
    }
}

pub async fn create(Form(payload): Form<CreateSubjectControllerModel>) -> impl IntoResponse {
    match subject::service::SERVICE
        .save(
            &payload.code,
            &payload.name,
            &payload.program,
            payload.courses.iter().map(|s| s.as_str()).collect(),
        )
        .await
    {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(_) => Redirect::to("/subjects").into_response(),
    }
}
