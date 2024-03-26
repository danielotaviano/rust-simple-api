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

    pub async fn save(&self, name: &str) -> Result<Course, String> {
        let course = Course::new(name);
        self.repository.save(&course).await
    }

    pub async fn edit(&self, id: &str, name: &str) -> Result<Course, String> {
        let course = Course::new_with_id(id, name);
        self.repository.save(&course).await
    }

    pub async fn delete(&self, course_id: String) -> Result<(), String> {
        let exists_student = student::service::SERVICE
            .list_students_by_course_id(&course_id)
            .await
            .expect("Error when trying to list students")
            .len()
            > 0;

        if exists_student {
            return Err(
                "Unable to delete the course because exists student that is enrolled".to_string(),
            );
        }

        self.repository.delete(&course_id).await
    }

    pub async fn get_course_by_id(&self, course_id: &String) -> Result<Option<Course>, String> {
        self.repository.get_by_id(&course_id).await
    }

    pub async fn list_courses(&self) -> Result<Vec<Course>, String> {
        self.repository.list().await
    }
}
