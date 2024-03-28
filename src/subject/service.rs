use super::{model::Subject, repository::Repository};
use crate::course::model::Course;
use once_cell::sync::Lazy;
use std::error::Error;

pub static SERVICE: Lazy<Service> = Lazy::new(Service::new);

pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn new() -> Self {
        Service {
            repository: Repository::new(),
        }
    }

    pub async fn save(
        &self,
        code: &str,
        name: &str,
        program: &str,
        courses_id: Vec<&str>,
    ) -> Result<Subject, Box<dyn Error>> {
        let subject = Subject::new(code, name, program);
        self.repository.save(&subject, courses_id).await
    }

    pub async fn list_with_courses(&self) -> Result<Vec<(Subject, Vec<Course>)>, Box<dyn Error>> {
        self.repository.list_with_courses().await
    }
}
