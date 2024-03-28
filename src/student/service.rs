use std::error::Error;

use once_cell::sync::Lazy;
use serde::Serialize;

use crate::{
    avatar::model::Avatar,
    course::{self, model::Course},
    subject::model::Subject,
};

use super::{model::Student, repository::Repository};

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
        operational_systems: Vec<&String>,
    ) -> Result<Student, Box<dyn Error>> {
        let course = match course::service::SERVICE.get_course_by_id(&course).await? {
            None => return Err("Course does not exists!".into()),
            Some(course) => course,
        };

        let student = Student::new(
            first_name,
            last_name,
            &course.get_id(),
            language,
            email,
            operational_systems,
        );

        let student = self.repository.save(student).await?;

        Ok(student)
    }

    pub async fn delete(&self, student_id: String) -> Result<(), Box<dyn Error>> {
        self.repository.delete(student_id).await
    }

    pub async fn get_student_by_id(
        &self,
        student_id: &str,
    ) -> Result<Option<Student>, Box<dyn Error>> {
        self.repository.get_by_id(student_id).await
    }

    pub async fn list_students_with_avatar(
        &self,
    ) -> Result<Vec<(Student, Option<Avatar>)>, Box<dyn Error>> {
        self.repository.list_student_with_avatar().await
    }

    pub async fn list_group_by(
        &self,
        group_by: &GroupBy,
    ) -> Result<Vec<(String, Vec<Student>)>, Box<dyn Error>> {
        match group_by {
            GroupBy::COURSE => self.repository.list_students_group_by_course().await,
            GroupBy::LANGUAGE => self.repository.list_students_group_by_language().await,
            GroupBy::OS => self.repository.list_students_group_by_os().await,
        }
    }

    pub async fn list_students_by_course_id(
        &self,
        course_id: &String,
    ) -> Result<Vec<Student>, Box<dyn Error>> {
        self.repository.list_students_by_course_id(course_id).await
    }

    pub async fn list_students_that_doesnt_have_avatar(
        &self,
    ) -> Result<Vec<Student>, Box<dyn Error>> {
        self.repository.list_student_that_doesnt_have_avatar().await
    }

    pub async fn get_student_with_course_and_subjects(
        &self,
        student_id: &str,
    ) -> Result<(Student, Course, Vec<Subject>), Box<dyn Error>> {
        self.repository
            .get_student_with_course_and_subjects(student_id)
            .await
    }
}
