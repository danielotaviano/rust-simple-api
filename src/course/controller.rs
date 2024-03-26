use super::service::SERVICE;
use crate::view::get_template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::Form;
use minijinja::render;
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
    let courses = SERVICE.list_courses().await;

    if let Err(_) = courses {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let template = get_template("course/list").unwrap();
    let r = render!(template, courses => courses.unwrap());
    Html(r).into_response()
}

pub async fn create_course_form_html() -> Html<String> {
    let template = get_template("course/create").unwrap();
    let r = render!(template);
    Html(r)
}

pub async fn edit_course_form_html(Path(course_id): Path<String>) -> impl IntoResponse {
    let course = SERVICE.get_course_by_id(&course_id).await;

    if let Err(_) = course {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let template = get_template("course/edit").unwrap();
    let r = render!(template, course => course.unwrap());
    Html(r).into_response()
}

pub async fn show_course_html(Path(course_id): Path<String>) -> impl IntoResponse {
    let course = SERVICE.get_course_by_id(&course_id).await;

    if let Err(_) = course {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let template = get_template("course/show").unwrap();
    let r = render!(template, course => course.unwrap());

    Html(r).into_response()
}

pub async fn create_course(Form(course): Form<CreateCourseControllerModel>) -> impl IntoResponse {
    let r = SERVICE.save(&course.name).await;

    if let Err(_) = r {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/courses").into_response()
}

pub async fn edit_course(
    Path(course_id): Path<String>,
    Form(course): Form<EditCourseControllerModel>,
) -> impl IntoResponse {
    let r = SERVICE.edit(&course_id, &course.name).await;

    if let Err(_) = r {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/courses").into_response()
}

pub async fn delete_course(Path(course_id): Path<String>) -> impl IntoResponse {
    let result = SERVICE.delete(course_id).await;

    match result {
        Err(message) => (StatusCode::BAD_REQUEST, message).into_response(),
        Ok(_) => (StatusCode::OK).into_response(),
    }
}
