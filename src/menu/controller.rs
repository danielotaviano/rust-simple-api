use axum::response::Html;
use minijinja::render;

use crate::view::get_template;

pub async fn render_student_list() -> Html<String> {
    let template = get_template("menu").unwrap();
    let r = render!(template);

    Html(r)
}
