use pf_diesel::*;
use std::io::{stdin, Read};
use pf_diesel::crud::*;
use pf_diesel::dev::establish_connection;

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
}