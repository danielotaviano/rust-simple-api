use crate::{course::model::Course, infra};
use sqlx::{Pool, Postgres};

pub struct Repository {
    database: &'static Pool<Postgres>,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            database: infra::db::DB_POOL.get().expect("Unable to get DB_POOL"),
        }
    }

    pub async fn delete(&self, course_id: &str) -> Result<(), String> {
        sqlx::query!(
            r#"
            DELETE FROM course WHERE id = $1
            "#,
            course_id
        )
        .execute(self.database)
        .await
        .expect("Error when trying to delete a course");

        Ok(())
    }

    pub async fn get_by_id(&self, course_id: &str) -> Result<Option<Course>, String> {
        let course = sqlx::query_as!(
            Course,
            r#"
                SELECT id, name FROM course WHERE id = $1
            "#,
            course_id
        )
        .fetch_optional(self.database)
        .await
        .expect("Error when trying to get a course");

        Ok(course)
    }

    pub async fn list(&self) -> Result<Vec<Course>, String> {
        let subjects = sqlx::query_as!(
            Course,
            r#"
            SELECT id, name FROM course
            "#,
        )
        .fetch_all(self.database)
        .await
        .expect("Error when trying to get subjects");

        Ok(subjects)
    }

    pub async fn save(&self, course: &Course) -> Result<Course, String> {
        let tx_result = self.database.begin().await;

        if let Err(_) = tx_result {
            return Err("Error when try to open a new transaction".to_string());
        }

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
