// @generated automatically by Diesel CLI.

diesel::table! {
    class (id) {
        id -> Int4,
        title -> Varchar,
    }
}

diesel::table! {
    club (id) {
        id -> Int4,
        title -> Varchar,
    }
}

diesel::table! {
    course (id) {
        id -> Int4,
        title -> Varchar,
    }
}

diesel::table! {
    student (id) {
        id -> Int4,
        title -> Varchar,
        class_id -> Int4,
        club_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    class,
    club,
    course,
    student,
);
