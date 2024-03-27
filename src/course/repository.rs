use crate::{course::model::Course, infra};
use sqlx::{Pool, Postgres};
use std::error::Error;

pub struct Repository {
    database: &'static Pool<Postgres>,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            database: infra::db::DB_POOL.get().expect("Unable to get DB_POOL"),
        }
    }

    pub async fn delete(&self, course_id: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
            DELETE FROM course WHERE id = $1
            "#,
            course_id
        )
        .execute(self.database)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(&self, course_id: &str) -> Result<Option<Course>, Box<dyn Error>> {
        let course = sqlx::query_as!(
            Course,
            r#"
                SELECT id, name FROM course WHERE id = $1
            "#,
            course_id
        )
        .fetch_optional(self.database)
        .await?;

        Ok(course)
    }

    pub async fn list(&self) -> Result<Vec<Course>, Box<dyn Error>> {
        let subjects = sqlx::query_as!(
            Course,
            r#"
            SELECT id, name FROM course
            "#,
        )
        .fetch_all(self.database)
        .await?;

        Ok(subjects)
    }

    pub async fn save(&self, course: &Course) -> Result<Course, Box<dyn Error>> {
        sqlx::query!(
            r#"
            INSERT INTO course (id, name)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET name=excluded.name
            "#,
            course.get_id(),
            course.get_name(),
        )
        .execute(self.database)
        .await
        .expect("Error when trying to add a subject");

        Ok(course.clone())
    }
}
