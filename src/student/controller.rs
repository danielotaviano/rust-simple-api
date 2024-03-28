use super::model::Student;
use super::service::SERVICE;
use crate::avatar::model::Avatar;
use crate::course::model::Course;
use crate::custom::HtmlResponse;
use crate::subject::model::Subject;
use crate::view::render_template;
use crate::{course, student::service::GroupBy};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::Form;
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

#[derive(Serialize)]
pub struct CreateStudentHtmlControllerModel {
    courses: Vec<Course>,
    os: Vec<String>,
}

#[derive(Serialize)]
pub struct ShowStudentHtmlControllerModel {
    student: Student,
    course: Course,
    subjects: Vec<Subject>,
}

#[derive(Serialize)]
pub struct ListStudentGroupByHtmlControllerModel {
    name: String,
    students: Vec<Student>,
}

pub async fn student_list_html() -> impl IntoResponse {
    let students_with_avatar = match SERVICE.list_students_with_avatar().await {
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(students_with_avatar) => students_with_avatar,
    };

    let students_struct: Vec<ListStudentControllerModel> = students_with_avatar
        .into_iter()
        .map(|student| ListStudentControllerModel {
            student: student.0,
            avatar: student.1,
        })
        .collect();

    render_template("student/list", students_struct.into()).to_html_response()
}

pub async fn create_student_form_html() -> impl IntoResponse {
    match course::service::SERVICE.list_courses().await {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(courses) => {
            let os = vec!["OSX", "Windows", "Linux"];

            let context = CreateStudentHtmlControllerModel {
                courses,
                os: os.into_iter().map(String::from).collect(),
            };

            render_template("student/create", context.into()).to_html_response()
        }
    }
}

pub async fn show_student_html(Path(student_id): Path<String>) -> impl IntoResponse {
    let student = match SERVICE
        .get_student_with_course_and_subjects(&student_id)
        .await
    {
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(student) => student,
    };

    let context = ShowStudentHtmlControllerModel {
        student: student.0,
        course: student.1,
        subjects: student.2,
    };

    render_template("student/show", context.into()).to_html_response()
}

pub async fn list_student_group_by_html(Query(q): Query<GroupByQueryParam>) -> impl IntoResponse {
    let entity_enum = match q.entity.as_str() {
        "course" => GroupBy::COURSE,
        "language" => GroupBy::LANGUAGE,
        "os" => GroupBy::OS,
        _ => return (StatusCode::BAD_REQUEST, "Invalid entity to group").into_response(),
    };

    match SERVICE.list_group_by(&entity_enum).await {
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        Ok(entities) => {
            let context: Vec<ListStudentGroupByHtmlControllerModel> = entities
                .into_iter()
                .map(|entity| ListStudentGroupByHtmlControllerModel {
                    name: entity.0,
                    students: entity.1,
                })
                .collect();

            render_template("student/group-by", context.into()).to_html_response()
        }
    }
}

pub async fn create_student(
    Form(student): Form<CreateStudentControllerModel>,
) -> impl IntoResponse {
    match SERVICE
        .save(
            &student.first_name,
            &student.last_name,
            &student.course,
            &student.language,
            &student.email,
            student.operational_systems.iter().collect(),
        )
        .await
    {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(_) => Redirect::to("/students").into_response(),
    }
}

pub async fn delete_student(Path(student_id): Path<String>) -> impl IntoResponse {
    match SERVICE.delete(student_id).await {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(_) => StatusCode::OK.into_response(),
    }
}
