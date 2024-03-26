-- Add migration script here
CREATE TABLE student (
    id VARCHAR PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    course_id VARCHAR NOT NULL,
    language VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    operational_systems _text NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (course_id) REFERENCES course(id)
);
