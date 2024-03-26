use futures::future::join_all;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use minijinja::render;
use serde::{Deserialize, Serialize};

use crate::{
    avatar,
    student::{self, model::Student},
    view::get_template,
};

use super::model::Avatar;

#[derive(Debug, Deserialize)]
pub struct CreateAvatarControllerModel {
    name: String,
    student: String,
}

#[derive(Serialize)]
pub struct ListAvatarControllerModel {
    avatar: Avatar,
    student: Student,
}

pub async fn list_avatar_html() -> impl IntoResponse {
    let avatars = avatar::service::SERVICE.list().await;

    if let Err(msg) = avatars {
        return (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response();
    }

    let mut students_futures = Vec::new();
    for avatar in avatars.as_ref().unwrap() {
        let student_id = avatar.student_id.clone();
        let student_future = async move {
            student::service::SERVICE
                .get_student_by_id(&student_id)
                .await
                .unwrap()
                .unwrap()
        };
        students_futures.push(student_future);
    }

    let students = join_all(students_futures).await;

    let avatars_with_student: Vec<ListAvatarControllerModel> = avatars
        .as_ref()
        .unwrap()
        .into_iter()
        .map(|a| ListAvatarControllerModel {
            avatar: a.clone(),
            student: students
                .clone()
                .into_iter()
                .find(|s| s.get_id() == a.get_student_id())
                .unwrap(),
        })
        .collect();

    let template = get_template("avatar/list").unwrap();
    let r = render!(template, avatars => avatars_with_student);

    Html(r).into_response()
}

pub async fn create_avatar_html() -> impl IntoResponse {
    let students = student::service::SERVICE
        .list_students_that_doesnt_have_avatar()
        .await
        .expect("Error when try to retrive students");

    let template = get_template("avatar/create").unwrap();
    let r = render!(template, students => students);

    Html(r).into_response()
}

pub async fn create_avatar(Form(payload): Form<CreateAvatarControllerModel>) -> impl IntoResponse {
    let student_exists = student::service::SERVICE
        .get_student_by_id(&payload.student)
        .await;

    if let Err(_) = student_exists {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    let already_exists_avatar = avatar::service::SERVICE
        .get_by_student_id(&payload.student)
        .await;

    if let Ok(_) = already_exists_avatar {
        return (
            StatusCode::BAD_REQUEST,
            "This student already has a avatar!",
        )
            .into_response();
    }

    match student_exists.unwrap() {
        None => (StatusCode::UNPROCESSABLE_ENTITY, "student does not exists").into_response(),
        Some(_) => {
            let result = avatar::service::SERVICE
                .save(&payload.name, &payload.student)
                .await;

            if let Err(msg) = result {
                return (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response();
            }

            Redirect::to("/avatars").into_response()
        }
    }
}
