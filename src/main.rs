mod avatar;
mod course;
mod infra;
mod menu;
mod student;
mod subject;
mod view;

use std::fmt::Error;

use axum::{
    routing::{delete, get},
    Router,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    infra::db::start_connection().await.unwrap();

    let app = Router::new()
        .route("/", get(menu::controller::render_student_list))
        .route("/students", get(student::controller::student_list_html))
        .route(
            "/students/group-by",
            get(student::controller::list_student_group_by_html),
        )
        .route(
            "/student/:student_id",
            delete(student::controller::delete_student).get(student::controller::show_student_html),
        )
        .route(
            "/student/create",
            get(student::controller::create_student_form_html)
                .post(student::controller::create_student),
        )
        .route("/courses", get(course::controller::course_list_html))
        .route(
            "/course/:course_id",
            get(course::controller::show_course_html).delete(course::controller::delete_course),
        )
        .route(
            "/course/create",
            get(course::controller::create_course_form_html)
                .post(course::controller::create_course),
        )
        .route(
            "/course/:course_id/edit",
            get(course::controller::edit_course_form_html).post(course::controller::edit_course),
        )
        .route(
            "/avatar/create",
            get(avatar::controller::create_avatar_html).post(avatar::controller::create_avatar),
        )
        .route("/avatars", get(avatar::controller::list_avatar_html))
        .route(
            "/subject/create",
            get(subject::controller::create_html).post(subject::controller::create),
        )
        .route("/subjects", get(subject::controller::list_html));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
