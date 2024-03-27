use sqlx::{Pool, Postgres};

use crate::infra::{self};

use super::model::Avatar;

pub struct Repository {
    database: &'static Pool<Postgres>,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            database: infra::db::DB_POOL.get().expect("Unable to get DB_POOL"),
        }
    }

    pub async fn save(&self, avatar: &Avatar) -> Result<Avatar, String> {
        sqlx::query!(
            r#"
            INSERT INTO avatar (id, fantasy_name, student_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET fantasy_name=excluded.fantasy_name, student_id=excluded.student_id
            "#,
            avatar.get_id(),
            avatar.get_fantasy_name(),
            avatar.get_student_id()
        )
        .execute(self.database)
        .await.map_err(|e| e.to_string())?;

        Ok(avatar.clone())
    }

    pub async fn get_by_student_id(&self, student_id: &str) -> Result<Option<Avatar>, String> {
        sqlx::query_as!(
            Avatar,
            r#"
            SELECT id, fantasy_name, student_id
            FROM avatar
            WHERE 
                student_id = $1
            "#,
            student_id
        )
        .fetch_optional(self.database)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn list(&self) -> Result<Vec<Avatar>, String> {
        sqlx::query_as!(
            Avatar,
            r#"
            SELECT id, fantasy_name, student_id
            FROM avatar
            "#
        )
        .fetch_all(self.database)
        .await
        .map_err(|e| e.to_string())
    }
}
