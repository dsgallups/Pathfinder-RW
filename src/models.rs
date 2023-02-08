use diesel::prelude::*;

#[derive(Queryable)]
pub struct University {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Queryable)]
pub struct Subdivision {
    pub id: i32,
    pub name: String,
    pub university: Option<University>
}