use std::error::Error;

use crate::student;
use once_cell::sync::Lazy;

use super::{model::Course, repository::Repository};

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

    pub async fn save(&self, name: &str) -> Result<Course, Box<dyn Error>> {
        let course = Course::new(name);
        self.repository.save(&course).await
    }

    pub async fn edit(&self, id: &str, name: &str) -> Result<Course, Box<dyn Error>> {
        let course = Course::new_with_id(id, name);
        self.repository.save(&course).await
    }

    pub async fn delete(&self, course_id: String) -> Result<(), Box<dyn Error>> {
        let students = student::service::SERVICE
            .list_students_by_course_id(&course_id)
            .await?;

        match students.is_empty() {
            true => self.repository.delete(&course_id).await,
            false => {
                Err("Unable to delete the course because exists student that is enrolled".into())
            }
        }
    }

    pub async fn get_course_by_id(
        &self,
        course_id: &str,
    ) -> Result<Option<Course>, Box<dyn Error>> {
        self.repository.get_by_id(&course_id).await
    }

    pub async fn list_courses(&self) -> Result<Vec<Course>, Box<dyn Error>> {
        self.repository.list().await
    }
}
