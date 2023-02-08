use diesel::prelude::*;
use crate::schema::university;

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


#[derive(Insertable)]
#[diesel(table_name = university)]
pub struct NewUniversity<'a> {
    pub name: &'a str,
    pub description: &'a str
}