use nanoid::nanoid;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Avatar {
    pub id: String,
    pub student_id: String,
    pub fantasy_name: String,
}

impl Avatar {
    pub fn new(fantasy_name: &str, student_id: &str) -> Self {
        Self {
            id: Self::generate_id(),
            fantasy_name: fantasy_name.to_string(),
            student_id: student_id.to_string(),
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

    pub fn get_fantasy_name(&self) -> &String {
        &self.fantasy_name
    }

    pub fn get_student_id(&self) -> &String {
        &self.student_id
    }
}
