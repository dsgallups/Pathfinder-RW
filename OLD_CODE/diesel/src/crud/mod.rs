use diesel::prelude::*;
use crate::schema::university;
use crate::models::{University, NewUniversity};

pub fn create_university(conn: &mut PgConnection, name: &str, description: &str) -> University {

    let new_univ = NewUniversity { name, description };

    diesel::insert_into(university::table)
        .values(&new_univ)
        .get_result(conn)
        .expect("Error in saving new university")
}