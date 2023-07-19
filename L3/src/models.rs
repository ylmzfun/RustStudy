use diesel::prelude::*;
use crate::schema::student;
use crate::schema::class;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::student)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Student {
    pub id: i32,
    pub title: String,
    pub class_id: i32,
    pub club_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = student)]
pub struct NewStudent<'a> {
    pub title: &'a str,
    pub class_id: &'a i32,
    pub club_id: &'a i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::class)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Class {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = class)]
pub struct NewClass<'a> {
    pub title: &'a str
}