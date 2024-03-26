use once_cell::sync::Lazy;

use super::{model::Avatar, repository::Repository};

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

    pub async fn save(&self, fantasy_name: &str, student_id: &str) -> Result<Avatar, String> {
        let avatar = Avatar::new(fantasy_name, student_id);
        self.repository.save(&avatar).await
    }

    pub async fn list(&self) -> Result<Vec<Avatar>, String> {
        self.repository.list().await
    }

    pub async fn get_by_student_id(&self, student_id: &str) -> Result<Option<Avatar>, String> {
        self.repository.get_by_student_id(student_id).await
    }
}
