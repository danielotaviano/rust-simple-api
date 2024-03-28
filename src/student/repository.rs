use std::{error::Error, vec};

use sqlx::{Pool, Postgres};

use super::model::Student;
use crate::{avatar::model::Avatar, course::model::Course, infra, subject::model::Subject};

pub struct Repository {
    database: &'static Pool<Postgres>,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            database: infra::db::DB_POOL.get().expect("Unable to get DB_POOL"),
        }
    }

    pub async fn save(&self, student: Student) -> Result<Student, Box<dyn Error>> {
        let updated_student = sqlx::query_as!(
            Student,
            r#"
            INSERT INTO student (id, first_name, last_name, course_id, language, email, operational_systems)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET first_name=excluded.first_name, last_name=excluded.last_name, course_id=excluded.course_id, language=excluded.language, email=excluded.email, operational_systems=excluded.operational_systems
            RETURNING id, first_name, last_name, course_id, language, email, operational_systems
            "#,
            student.get_id(),
            student.get_first_name(),
            student.get_last_name(),
            student.get_course(),
            student.get_language(),
            student.get_email(),
            student.get_operational_systems()
        ).fetch_one(self.database).await?;

        Ok(updated_student)
    }

    pub async fn list_students_by_course_id(
        &self,
        course_id: &String,
    ) -> Result<Vec<Student>, Box<dyn Error>> {
        let students = sqlx::query_as!(
            Student,
            r#"
            SELECT s.id, first_name, last_name, course_id, language, email, operational_systems
            FROM student s
            INNER JOIN course c on c.id = s.course_id
            WHERE c.id = $1
            "#,
            course_id
        )
        .fetch_all(self.database)
        .await?;

        Ok(students)
    }

    pub async fn list_students_group_by_course(
        &self,
    ) -> Result<Vec<(String, Vec<Student>)>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            select
                c.*, 
                JSON_agg(s.*) "students"
            from
                course c
            inner join student s on
                s.course_id = c.id
            group by
                c.id
            "#,
        )
        .fetch_all(self.database)
        .await?;

        let courses_with_students: Vec<(String, Vec<Student>)> = rows
            .into_iter()
            .map(|row| {
                let students: Vec<Student> = match row.students {
                    None => None,
                    Some(students) => serde_json::from_value(students).ok(),
                }
                .unwrap_or(vec![]);

                (row.name, students)
            })
            .collect();

        Ok(courses_with_students)
    }

    pub async fn list_students_group_by_language(
        &self,
    ) -> Result<Vec<(String, Vec<Student>)>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            select
                s."language",
                json_agg(s.*) "students"
            from
                student s
            group by
                s."language"
            "#,
        )
        .fetch_all(self.database)
        .await?;

        let languages_with_students: Vec<(String, Vec<Student>)> = rows
            .into_iter()
            .map(|row| {
                let students: Vec<Student> = match row.students {
                    None => None,
                    Some(students) => serde_json::from_value(students).ok(),
                }
                .unwrap_or(vec![]);

                (row.language, students)
            })
            .collect();

        Ok(languages_with_students)
    }

    pub async fn list_students_group_by_os(
        &self,
    ) -> Result<Vec<(String, Vec<Student>)>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            select
                os "os",
                JSON_agg(s.*) "students"
            from
                student s,
                unnest(s.operational_systems) "os"
            group by
                os
            "#,
        )
        .fetch_all(self.database)
        .await?;

        let os_with_students: Vec<(String, Vec<Student>)> = rows
            .into_iter()
            .map(|row| {
                let students: Vec<Student> = match row.students {
                    None => None,
                    Some(students) => serde_json::from_value(students).ok(),
                }
                .unwrap_or(vec![]);

                (row.os.unwrap_or("unknow".to_string()), students)
            })
            .collect();

        Ok(os_with_students)
    }

    pub async fn delete(&self, student_id: String) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
            DELETE FROM student WHERE id = $1
            "#,
            student_id
        )
        .execute(self.database)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(&self, student_id: &str) -> Result<Option<Student>, Box<dyn Error>> {
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
        .await?;

        Ok(student)
    }

    pub async fn list_student_that_doesnt_have_avatar(
        &self,
    ) -> Result<Vec<Student>, Box<dyn Error>> {
        let students = sqlx::query_as!(
            Student,
            r#"
            SELECT
                s.id,
                first_name,
                last_name,
                course_id,
                language,
                email,
                operational_systems
            FROM
                student s
            LEFT JOIN avatar a ON
                a.student_id = s.id
            WHERE
                a.student_id IS NULL
            "#
        )
        .fetch_all(self.database)
        .await?;

        Ok(students)
    }

    pub async fn list_student_with_avatar(
        &self,
    ) -> Result<Vec<(Student, Option<Avatar>)>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            select
                row_to_json(s.*) "student",
                row_to_json(a.*) "avatar"
            from
                student s
            left join avatar a on
                a.student_id = s.id
            "#
        )
        .fetch_all(self.database)
        .await?;

        let students_with_avatar: Vec<_> = rows
            .into_iter()
            .filter_map(|row| {
                row.student.and_then(|student| {
                    let student: Option<Student> = serde_json::from_value(student).ok();
                    let avatar: Option<Avatar> = row
                        .avatar
                        .and_then(|avatar| serde_json::from_value(avatar).ok());

                    student.map(|student| (student, avatar))
                })
            })
            .collect();

        Ok(students_with_avatar)
    }

    pub async fn get_student_with_course_and_subjects(
        &self,
        student_id: &str,
    ) -> Result<(Student, Course, Vec<Subject>), Box<dyn Error>> {
        let row = sqlx::query!(
            r#"
            select
                row_to_json(s.*) "student",
                row_to_json(c.*) "course",
                json_agg(s2.*) "subjects"
            from
                student s
            inner join course c on
                c.id = s.course_id
            left join subject_course sc on
                sc.course_id = c.id
            left join subject s2 on
                s2.id = sc.subject_id
            WHERE s.id = $1
            group by
                s.id,
                c.id
            "#,
            student_id
        )
        .fetch_one(self.database)
        .await?;

        match (row.student, row.course) {
            (Some(student), Some(course)) => {
                let student: Student = serde_json::from_value(student)?;
                let course: Course = serde_json::from_value(course)?;
                let subjects: Vec<Subject> =
                    serde_json::from_value(row.subjects.unwrap_or_default())
                        .ok()
                        .unwrap_or(vec![]);

                Ok((student, course, subjects))
            }
            _ => Err("Student or course is None and can't be".into()),
        }
    }
}
