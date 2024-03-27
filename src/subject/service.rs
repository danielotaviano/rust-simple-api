use crate::{course, student};
use once_cell::sync::Lazy;

use super::{
    model::Subject,
    repository::{Repository, SubjectWithCourses},
};

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
    ) -> Result<Subject, String> {
        let subject = Subject::new(code, name, program);
        self.repository.save(&subject, courses_id).await
    }

    pub async fn list(&self) -> Result<Vec<Subject>, String> {
        self.repository.list().await
    }

    pub async fn list_with_courses(&self) -> Result<Vec<SubjectWithCourses>, String> {
        self.repository.list_with_courses().await
    }
    pub async fn list_by_course_id(&self, course_id: &str) -> Result<Vec<Subject>, String> {
        self.repository.list_by_course_id(course_id).await
    }
}
