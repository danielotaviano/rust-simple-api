use super::{model::Avatar, repository::Repository};
use crate::student::{self, model::Student};
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
        fantasy_name: &str,
        student_id: &str,
    ) -> Result<Avatar, Box<dyn Error>> {
        if let None = student::service::SERVICE
            .get_student_by_id(student_id)
            .await?
        {
            return Err("Student does not exists".into());
        }

        if let Some(_) = SERVICE.get_by_student_id(&student_id).await? {
            return Err("Student already has an avatar!".into());
        }

        let avatar = Avatar::new(fantasy_name, student_id);
        self.repository.save(&avatar).await
    }

    pub async fn get_by_student_id(
        &self,
        student_id: &str,
    ) -> Result<Option<Avatar>, Box<dyn Error>> {
        self.repository.get_by_student_id(student_id).await
    }

    pub async fn list_with_students(&self) -> Result<Vec<(Avatar, Student)>, Box<dyn Error>> {
        self.repository.list_with_student().await
    }
}
