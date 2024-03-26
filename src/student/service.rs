use once_cell::sync::Lazy;
use serde::Serialize;

use super::{
    model::Student,
    repository::{ListStudentsGroupBy, Repository},
};

#[derive(Debug, Serialize)]
pub enum GroupBy {
    COURSE,
    LANGUAGE,
    OS,
}

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
        first_name: &str,
        last_name: &str,
        course: &str,
        language: &str,
        email: &str,
        operational_systems: Vec<String>,
    ) -> Result<Student, String> {
        self.repository
            .save(
                first_name,
                last_name,
                course,
                language,
                email,
                operational_systems,
            )
            .await
    }

    pub async fn delete(&self, student_id: String) -> Result<(), String> {
        self.repository.delete(student_id).await
    }

    pub async fn get_student_by_id(&self, student_id: &str) -> Result<Option<Student>, String> {
        self.repository.get_by_id(student_id).await
    }

    pub async fn list_students(&self) -> Result<Vec<Student>, String> {
        self.repository.list().await
    }

    pub async fn list_group_by(
        &self,
        group_by: &GroupBy,
    ) -> Result<Vec<ListStudentsGroupBy>, String> {
        match group_by {
            GroupBy::COURSE => self.repository.list_students_group_by_course().await,
            GroupBy::LANGUAGE => self.repository.list_students_group_by_language().await,
            GroupBy::OS => self.repository.list_students_group_by_os().await,
        }
    }

    pub async fn list_students_by_course_id(
        &self,
        course_id: &String,
    ) -> Result<Vec<Student>, String> {
        self.repository.list_students_by_course_id(course_id).await
    }

    pub async fn list_students_that_doesnt_have_avatar(&self) -> Result<Vec<Student>, String> {
        self.repository.list_student_that_doesnt_have_avatar().await
    }
}
