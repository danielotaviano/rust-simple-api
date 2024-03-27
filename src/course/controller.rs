use super::service::SERVICE;
use crate::{custom::HtmlResponse, view::render_template};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::Form;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourseControllerModel {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EditCourseControllerModel {
    name: String,
}

pub async fn course_list_html() -> impl IntoResponse {
    match SERVICE.list_courses().await {
        Ok(courses) => render_template("course/list", courses.into()).to_html_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_course_form_html() -> impl IntoResponse {
    render_template("course/create", ().into()).to_html_response()
}

pub async fn edit_course_form_html(Path(course_id): Path<String>) -> impl IntoResponse {
    match SERVICE.get_course_by_id(&course_id).await {
        Ok(course) => render_template("course/edit", course).to_html_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn show_course_html(Path(course_id): Path<String>) -> impl IntoResponse {
    match SERVICE.get_course_by_id(&course_id).await {
        Ok(course) => render_template("course/show", course).to_html_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_course(Form(course): Form<CreateCourseControllerModel>) -> impl IntoResponse {
    match SERVICE.save(&course.name).await {
        Ok(_) => Redirect::to("/courses").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn edit_course(
    Path(course_id): Path<String>,
    Form(course): Form<EditCourseControllerModel>,
) -> impl IntoResponse {
    match SERVICE.edit(&course_id, &course.name).await {
        Ok(_) => Redirect::to("/courses").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_course(Path(course_id): Path<String>) -> impl IntoResponse {
    match SERVICE.delete(course_id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}
