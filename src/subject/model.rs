use nanoid::nanoid;

#[derive(Clone)]
pub struct Subject {
    pub id: String,
    pub code: String,
    pub name: String,
    pub program: String,
}

impl Subject {
    fn new(code: &str, name: &str, program: &str) -> Self {
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

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_code(&self) -> &str {
        self.code.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_program(&self) -> &str {
        self.program.as_str()
    }
}
