use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{NewUniversity, University};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_university(conn: &mut PgConnection, name: &str, description: &str) -> University {
    use crate::schema::university;

    let new_univ = NewUniversity { name, description };

    diesel::insert_into(university::table)
        .values(&new_univ)
        .get_result(conn)
        .expect("Error in saving new university")
}