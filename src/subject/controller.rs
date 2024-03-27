use axum::response::{Html, IntoResponse, Redirect};
use axum_extra::extract::Form;
use minijinja::render;
use serde::Deserialize;

use crate::{course, subject, view::get_template};

#[derive(Deserialize, Debug)]
pub struct CreateSubjectControllerModel {
    code: String,
    name: String,
    program: String,
    courses: Vec<String>,
}

pub async fn list_html() -> Html<String> {
    let payloads = super::service::SERVICE
        .list_with_courses()
        .await
        .expect("error when trying to get subjects");

    let template = get_template("subject/list").unwrap();
    let r = render!(template, payloads => payloads);

    Html(r)
}

pub async fn create_html() -> impl IntoResponse {
    let courses = course::service::SERVICE
        .list_courses()
        .await
        .expect("Error when trying to get courses");

    let template = get_template("subject/create").unwrap();
    let r = render!(template, courses => courses);

    Html(r)
}

pub async fn create(Form(payload): Form<CreateSubjectControllerModel>) -> impl IntoResponse {
    subject::service::SERVICE
        .save(
            &payload.code,
            &payload.name,
            &payload.program,
            payload.courses.iter().map(|s| s.as_str()).collect(),
        )
        .await
        .expect("Error when trying to create");

    Redirect::to("/subjects")
}
