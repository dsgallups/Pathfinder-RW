use diesel::prelude::*;
use pf_diesel::*;
use pf_diesel::schema::university::dsl::*;
use pf_diesel::models::{University};
use pf_diesel::dev::establish_connection;
fn main() {


    let conn = &mut establish_connection();
    let results = university
        .limit(5)
        .load::<University>(conn)
        .expect("Error Loading Universities");

    println!("Displaying {} universities", results.len());

    for u in results {
        println!("{}", u.name);
        println!("------------\n");
        println!("{}", u.description.unwrap_or("No Description".to_string()));
    }

}