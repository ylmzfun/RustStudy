-- Your SQL goes here
create table student
(

    id    SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    class_id int,
    club_id int
)
