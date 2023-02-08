use self::models::*;
use diesel::prelude::*;
use Pathfinder_RW::*;

fn main() {
    use self::schema::university::dsl::*;

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