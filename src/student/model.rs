use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Student {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub course_id: String,
    pub language: String,
    pub email: String,
    pub operational_systems: Vec<String>,
}

impl Student {
    pub fn new(
        first_name: &str,
        last_name: &str,
        course_id: &str,
        language: &str,
        email: &str,
        operational_systems: Vec<&String>,
    ) -> Self {
        Student {
            id: Student::generate_id(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            course_id: course_id.to_string(),
            language: language.to_string(),
            email: email.to_string(),
            operational_systems: operational_systems
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    fn generate_id() -> String {
        static ALPHABET: [char; 35] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'h', 'i', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'w', 'v', 'x', 'y',
            'z',
        ];

        nanoid!(10, &ALPHABET)
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_first_name(&self) -> &String {
        &self.first_name
    }

    pub fn get_last_name(&self) -> &String {
        &self.last_name
    }

    pub fn get_course(&self) -> &String {
        &self.course_id
    }

    pub fn get_language(&self) -> &String {
        &self.language
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_operational_systems(&self) -> &Vec<String> {
        &self.operational_systems
    }
}
