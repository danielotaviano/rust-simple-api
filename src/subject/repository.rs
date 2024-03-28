use std::error::Error;

use super::model::Subject;
use crate::{course::model::Course, infra};
use nanoid::nanoid;
use serde_json;
use sqlx::Row;
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

    fn generate_relation_id() -> String {
        static ALPHABET: [char; 35] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'h', 'i', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'w', 'v', 'x', 'y',
            'z',
        ];

        nanoid!(10, &ALPHABET)
    }

    pub async fn list_with_courses(&self) -> Result<Vec<(Subject, Vec<Course>)>, Box<dyn Error>> {
        let rows = sqlx::query(
            r#"
            select
                s.*,
                JSON_AGG(c.*) "courses"
            from
                subject s
            inner join subject_course on
                s.id = subject_course.subject_id
            inner join course c on
                subject_course.course_id = c.id
            group by
                s.id
            "#,
        )
        .fetch_all(self.database)
        .await?;

        let subject_with_courses: Vec<(Subject, Vec<Course>)> = rows
            .iter()
            .map(|row| {
                let subject = Subject {
                    id: row.get("id"),
                    code: row.get("code"),
                    name: row.get("name"),
                    program: row.get("program"),
                };

                let courses: Vec<Course> =
                    serde_json::from_value(row.get("courses")).unwrap_or(vec![]);

                (subject, courses)
            })
            .collect();

        Ok(subject_with_courses)
    }

    pub async fn save(
        &self,
        subject: &Subject,
        courses_id: Vec<&str>,
    ) -> Result<Subject, Box<dyn Error>> {
        let mut tx = match self.database.begin().await {
            Err(_) => return Err("Error when try to open a new transaction".into()),
            Ok(tx) => tx,
        };

        sqlx::query!(
            r#"
            INSERT INTO subject (id, code, name, program)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET code=excluded.code, name=excluded.name, program=excluded.program
            "#,
            subject.get_id(),
            subject.get_code(),
            subject.get_name(),
            subject.get_program()
        )
        .execute(&mut *tx)
        .await?;

        let mut relations_ids = vec![];
        relations_ids.resize_with(courses_id.len(), Repository::generate_relation_id);

        sqlx::query(
            r#"
            INSERT INTO subject_course (id, subject_id, course_id)
            SELECT * FROM UNNEST($1, $2, $3)
            RETURNING id, subject_id, course_id;
            "#,
        )
        .bind(relations_ids)
        .bind(vec![subject.get_id().clone(); courses_id.len()])
        .bind(courses_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(subject.clone())
    }
}
