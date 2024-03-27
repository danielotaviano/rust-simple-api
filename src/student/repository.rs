use std::collections::HashMap;

use serde::Serialize;
use sqlx::{Pool, Postgres};

use super::model::Student;
use crate::{course, infra};

#[derive(Clone, Serialize, Debug)]
pub struct ListStudentsGroupBy {
    name: String,
    students: Vec<Student>,
    total: usize,
}

pub struct Repository {
    database: &'static Pool<Postgres>,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            database: infra::db::DB_POOL.get().expect("Unable to get DB_POOL"),
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
        let student = Student::new(
            first_name,
            last_name,
            course,
            language,
            email,
            operational_systems,
        );

        let result = sqlx::query!(
            r#"
            INSERT INTO student (id, first_name, last_name, course_id, language, email, operational_systems)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET first_name=excluded.first_name, last_name=excluded.last_name, course_id=excluded.course_id, language=excluded.language, email=excluded.email, operational_systems=excluded.operational_systems
            "#,
            student.get_id(),
            student.get_first_name(),
            student.get_last_name(),
            student.get_course(),
            student.get_language(),
            student.get_email(),
            &student.get_operational_systems()
        ).execute(self.database).await;

        match result {
            Err(e) => Err(e.to_string()),
            _ => Ok(student),
        }
    }

    pub async fn list_students_by_course_id(
        &self,
        course_id: &String,
    ) -> Result<Vec<Student>, String> {
        let students = self
            .list()
            .await
            .expect("Error when trying to list students");

        Ok(students
            .into_iter()
            .filter(|s| s.get_course() == course_id)
            .collect())
    }

    pub async fn list_students_group_by_course(&self) -> Result<Vec<ListStudentsGroupBy>, String> {
        let students = self
            .list()
            .await
            .expect("Error when trying to list students");

        let mut grouped: HashMap<String, ListStudentsGroupBy> = HashMap::new();

        for student in students {
            let course_id = student.get_course();
            let course = course::service::SERVICE
                .get_course_by_id(&course_id)
                .await
                .expect("Error when trying to mount students by group")
                .expect("Course not found");
            let course_grouped =
                grouped
                    .entry(course_id.clone())
                    .or_insert_with(|| ListStudentsGroupBy {
                        name: course.get_name().clone(),
                        students: Vec::new(),
                        total: 0,
                    });

            course_grouped.students.push(student);
            course_grouped.total += 1;
        }

        Ok(grouped.into_iter().map(|(_, v)| v).collect())
    }

    pub async fn list_students_group_by_language(
        &self,
    ) -> Result<Vec<ListStudentsGroupBy>, String> {
        let students = self
            .list()
            .await
            .expect("Error when trying to list students");

        let mut grouped: HashMap<String, ListStudentsGroupBy> = HashMap::new();

        for student in students {
            let language = student.get_language();
            let language_grouped =
                grouped
                    .entry(language.clone())
                    .or_insert_with(|| ListStudentsGroupBy {
                        name: student.get_language().clone(),
                        students: Vec::new(),
                        total: 0,
                    });

            language_grouped.students.push(student);
            language_grouped.total += 1;
        }

        Ok(grouped.into_iter().map(|(_, v)| v).collect())
    }

    pub async fn list_students_group_by_os(&self) -> Result<Vec<ListStudentsGroupBy>, String> {
        let students = self
            .list()
            .await
            .expect("Error when trying to list students");

        let mut grouped: HashMap<String, ListStudentsGroupBy> = HashMap::new();

        for student in students {
            let oss = student.get_operational_systems();

            for os in oss {
                let os_grouped = grouped
                    .entry(os.clone())
                    .or_insert_with(|| ListStudentsGroupBy {
                        name: os.clone(),
                        students: Vec::new(),
                        total: 0,
                    });

                os_grouped.students.push(student.clone());
                os_grouped.total += 1;
            }
        }

        Ok(grouped.into_iter().map(|(_, v)| v).collect())
    }

    pub async fn delete(&self, student_id: String) -> Result<(), String> {
        let result = sqlx::query!(
            r#"
            DELETE FROM student WHERE id = $1
            "#,
            student_id
        )
        .execute(self.database)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_by_id(&self, student_id: &str) -> Result<Option<Student>, String> {
        let student = sqlx::query_as!(
            Student,
            r#"
            SELECT id, first_name, last_name, course_id, language, email, operational_systems
            FROM student
            WHERE id = $1
            "#,
            student_id
        )
        .fetch_optional(self.database)
        .await;

        match student {
            Ok(s) => Ok(s),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn list(&self) -> Result<Vec<Student>, String> {
        let students = sqlx::query_as!(
            Student,
            r#"
            SELECT id, first_name, last_name, course_id, language, email, operational_systems
            FROM student
            "#
        )
        .fetch_all(self.database)
        .await;

        match students {
            Ok(s) => Ok(s),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn list_student_that_doesnt_have_avatar(&self) -> Result<Vec<Student>, String> {
        let students = sqlx::query_as!(
            Student,
            r#"
            SELECT s.id, first_name, last_name, course_id, language, email, operational_systems
            FROM student s
            LEFT JOIN avatar a on a.student_id = s.id
            WHERE a.id is null 
            "#
        )
        .fetch_all(self.database)
        .await;

        match students {
            Ok(s) => Ok(s),
            Err(e) => Err(e.to_string()),
        }
    }
}
