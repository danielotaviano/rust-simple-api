-- Add migration script here


ALTER TABLE avatar ADD CONSTRAINT unique_student_id UNIQUE (student_id);
