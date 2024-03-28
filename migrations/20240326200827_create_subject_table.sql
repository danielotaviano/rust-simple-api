-- Add migration script here


CREATE TABLE subject (
    id VARCHAR PRIMARY KEY,
    code VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    program VARCHAR NOT NULL
);

CREATE TABLE subject_course (
    id VARCHAR PRIMARY KEY,
    subject_id VARCHAR NOT NULL,
    course_id VARCHAR NOT NULL,
    FOREIGN KEY (subject_id) REFERENCES subject(id),
    FOREIGN KEY (course_id) REFERENCES course(id),
    UNIQUE (subject_id, course_id)
);