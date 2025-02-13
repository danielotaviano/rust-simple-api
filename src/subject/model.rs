use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: String,
    pub code: String,
    pub name: String,
    pub program: String,
}

impl Subject {
    pub fn new(code: &str, name: &str, program: &str) -> Self {
        Self {
            id: Self::generate_id(),
            code: code.to_string(),
            name: name.to_string(),
            program: program.to_string(),
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

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_program(&self) -> &String {
        &self.program
    }
}
