use super::model::Student;
use super::service::SERVICE;
use crate::avatar::model::Avatar;
use crate::subject::model::Subject;
use crate::view::get_template;
use crate::{avatar, subject};
use crate::{course, student::service::GroupBy};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::Form;
use minijinja::render;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GroupByQueryParam {
    entity: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateStudentControllerModel {
    first_name: String,
    last_name: String,
    course: String,
    email: String,
    language: String,
    operational_systems: Vec<String>,
}

#[derive(Serialize)]
pub struct ListStudentControllerModel {
    student: Student,
    avatar: Option<Avatar>,
}

pub async fn student_list_html() -> impl IntoResponse {
    let students = SERVICE.list_students().await;

    if let Err(_) = students {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    let mut avatar_futures = Vec::new();

    for student in students.as_ref().unwrap() {
        let avatar = async move {
            let student_id = student.get_id().clone();
            avatar::service::SERVICE
                .get_by_student_id(&student_id)
                .await
                .unwrap()
        };
        avatar_futures.push(avatar);
    }

    let avatars = futures::future::join_all(avatar_futures).await;

    let payload: Vec<_> = students
        .as_ref()
        .unwrap()
        .into_iter()
        .map(|s| ListStudentControllerModel {
            student: s.clone(),
            avatar: avatars
                .clone()
                .into_iter()
                .find(|a| {
                    a.as_ref()
                        .map(|a| a.get_student_id() == s.get_id())
                        .unwrap_or(false)
                })
                .unwrap_or(None),
        })
        .collect();

    let template = get_template("student/list").unwrap();
    let r = render!(template, payloads => payload);
    Html(r).into_response()
}

pub async fn create_student_form_html() -> impl IntoResponse {
    let courses = course::service::SERVICE.list_courses().await;
    let os = vec!["OSX", "Windows", "Linux"];

    match courses {
        Ok(courses) => {
            let template = get_template("student/create").unwrap();
            let r = render!(template, courses => courses, opsys => os);
            Html(r).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn show_student_html(Path(student_id): Path<String>) -> impl IntoResponse {
    let student = SERVICE.get_student_by_id(&student_id).await;

    if let Err(_) = student {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    let student_result = student.unwrap();

    match student_result {
        None => (StatusCode::NOT_FOUND).into_response(),
        Some(student) => {
            let course = course::service::SERVICE
                .get_course_by_id(&student.get_course())
                .await
                .expect("error when trying to get course");

            let subjects: Option<Vec<Subject>> = match &course {
                None => None,
                Some(c) => Some(
                    subject::service::SERVICE
                        .list_by_course_id(&c.get_id())
                        .await
                        .expect("Error whwn trying to get subjects"),
                ),
            };

            let template = get_template("student/show").unwrap();
            let r = render!(template, student => student, course => course, subjects, subjects);

            Html(r).into_response()
        }
    }
}

pub async fn list_student_group_by_html(Query(q): Query<GroupByQueryParam>) -> impl IntoResponse {
    let entity_enum = match q.entity.as_str() {
        "course" => Some(GroupBy::COURSE),
        "language" => Some(GroupBy::LANGUAGE),
        "os" => Some(GroupBy::OS),
        _ => None,
    };

    if entity_enum.is_none() {
        return (StatusCode::BAD_REQUEST, "Invalid entity to group").into_response();
    }

    let payload = SERVICE.list_group_by(entity_enum.as_ref().unwrap()).await;

    if let Err(_) = payload {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    let template = get_template("student/group-by").unwrap();
    let r =
        render!(template, entities => payload.unwrap(), entity => entity_enum.as_ref().unwrap());

    Html(r).into_response()
}

pub async fn create_student(
    Form(student): Form<CreateStudentControllerModel>,
) -> impl IntoResponse {
    let course = course::service::SERVICE
        .get_course_by_id(&student.course)
        .await;

    if let Err(_) = course {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    let course_result = course.unwrap();
    if let None = course_result {
        return (StatusCode::UNPROCESSABLE_ENTITY).into_response();
    }

    let result = SERVICE
        .save(
            &student.first_name,
            &student.last_name,
            &student.course,
            &student.language,
            &student.email,
            student
                .operational_systems
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        )
        .await;

    match result {
        Ok(_) => Redirect::to("/students").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn delete_student(Path(student_id): Path<String>) -> impl IntoResponse {
    let result = SERVICE.delete(student_id).await;

    match result {
        Err(message) => (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
        Ok(_) => (StatusCode::OK).into_response(),
    }
}
