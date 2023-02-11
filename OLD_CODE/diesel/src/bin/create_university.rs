use pf_diesel::*;
use diesel::prelude::*;
use std::io::{stdin, Read};
use pf_diesel::dev::establish_connection;

use crate::models::{University, NewUniversity};
use crate::schema::university;

use crate::models::{Class, NewClass};
use crate::schema::class;
#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

fn main() {
    let conn = &mut establish_connection();

    let mut name = String::new();
    let mut description = String::new();

    println!("What do you want to name your university?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    println!(
        "\nOk! Writing {} (press {} when finished",
        name,
        EOF
    );

    stdin().read_to_string(&mut description).unwrap();

    let university = create_university(conn, name, &description);
    println!("\nSaved draft {} with id {}", name, university.id);

    //let class = 

    let class = create_class(conn, "testclass", 3);

    println!("\nSaved draft {} with id {}", class.name, class.id);
}

fn create_university(conn: &mut PgConnection, name: &str, description: &str) -> University {

    let new_univ = NewUniversity { name, description };

    diesel::insert_into(university::table)
        .values(&new_univ)
        .get_result(conn)
        .expect("Error in saving new university")
}

fn create_class(conn: &mut PgConnection, name: &str, credits: i32) -> Class {
    let new_class = NewClass { name, credits: &credits };

    diesel::insert_into(class::table)
        .values(&new_class)
        .get_result(conn)
        .expect("Error in saving new university")
}