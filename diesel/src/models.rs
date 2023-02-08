use diesel::prelude::*;
use crate::schema::university;
use crate::schema::class;

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

#[derive(Queryable)]
pub struct Class {
    pub id: i32,
    pub subject: Option<String>,
    pub course_no: Option<String>,
    pub credits: Option<i32>,
    pub pftype: String,
    pub title: Option<String>,
    pub description: String,
    //This will need to be a json in the future
    pub options: Option<String>
}

#[derive(Queryable)]
pub struct Component {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub pftype: String,
    pub class: Option<Class>,
    pub options: Option<String>
}

#[derive(Queryable)]
pub struct ComponentToComponent {
    pub id: i32,
    pub parent_id: Component,
    pub child_id: Component
}

#[derive(Queryable)]
pub struct Degree {
    pub id: i32,
    pub name: String,
    pub pftype: String,
    pub code: String,
    pub description: Option<String>,
    pub subdivision: Option<Subdivision>,

}

#[derive(Queryable)]
pub struct DegreeToComponent {
    pub id: i32,
    pub degree: Degree,
    pub component: Component
}


#[derive(Insertable)]
#[diesel(table_name = university)]
pub struct NewUniversity<'a> {
    pub name: &'a str,
    pub description: &'a str
}

#[derive(Insertable)]
#[diesel(table_name = class)]
pub struct NewClass<'a> {
    pub title: &'a str,
    pub credits: &'a i32
}