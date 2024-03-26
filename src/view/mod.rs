use std::fs;

pub fn get_template(path: &str) -> Result<&'static str, String> {
    let def_path = format!("src/view/{path}.jinja");

    let result = fs::read(def_path);

    if let Err(s) = result {
        return Err(s.to_string());
    }

    let content = result.unwrap();

    let raw_html = String::from_utf8(content);

    let static_content = match raw_html {
        Ok(html) => html,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let static_content: &'static str = Box::leak(static_content.into_boxed_str());
    Ok(static_content)
}
