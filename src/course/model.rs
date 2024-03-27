use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
}

impl Course {
    pub fn new(name: &str) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
        }
    }

    pub fn new_with_id(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
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

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
