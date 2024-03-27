use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};
use serde::{Deserialize, Serialize};

use crate::{
    avatar,
    custom::HtmlResponse,
    student::{self, model::Student},
    view::render_template,
};

use super::{model::Avatar, service::SERVICE};

#[derive(Debug, Deserialize)]
pub struct CreateAvatarControllerModel {
    name: String,
    student: String,
}

#[derive(Serialize)]
pub struct ListAvatarWithStudentControllerModel {
    avatar: Avatar,
    student: Student,
}

pub async fn list_avatar_html() -> impl IntoResponse {
    let avatars_with_students = match SERVICE.list_with_students().await {
        Ok(avatars) => avatars,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let avatars_with_students: Vec<ListAvatarWithStudentControllerModel> = avatars_with_students
        .into_iter()
        .map(|(avatar, student)| ListAvatarWithStudentControllerModel { avatar, student })
        .collect();

    render_template("avatar/list", avatars_with_students.into()).to_html_response()
}

pub async fn create_avatar_html() -> impl IntoResponse {
    match student::service::SERVICE
        .list_students_that_doesnt_have_avatar()
        .await
    {
        Ok(students) => render_template("avatar/create", students.into()).to_html_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_avatar(Form(payload): Form<CreateAvatarControllerModel>) -> impl IntoResponse {
    match avatar::service::SERVICE
        .save(&payload.name, &payload.student)
        .await
    {
        Ok(_) => Redirect::to("avatars").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
