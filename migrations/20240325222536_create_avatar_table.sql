-- Add migration script here


CREATE TABLE avatar (
    id VARCHAR PRIMARY KEY,
    fantasy_name VARCHAR NOT NULL,
    student_id VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES student(id),
    UNIQUE (student_id)
);