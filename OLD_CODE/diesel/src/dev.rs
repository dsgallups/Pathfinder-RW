
use std::process::{Command, Output};
use std::str;
use diesel::pg::PgConnection;
use diesel::{prelude::*, insert_into};
use dotenvy::dotenv;
use std::env;
//resets the DB tables (Drops and recreates)

use crate::models::{
    Class, 
    NewClass,
    Component,
    NewClassComponent
};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn reset_all_tables() -> Output {
    Command::new("diesel")
        .arg("database")
        .arg("reset")
        .output()
        .expect("Failed to reset tables. (diesel/src/dev.rs)")
}

pub fn insert_catalog() -> Result<String, String> {

    let conn = &mut establish_connection();

    let mut classes: Vec<Component> = Vec::new();
    let mut components: Vec<Component> = Vec::new();
    //insert Purdue

    //insert the CIT department (skip polytechnic because lazy)

    //insert classes
    push_classes(conn, &mut classes);
    push_components(conn, &mut components);
    
    Ok("Nice".to_string())
}

fn create_class(conn: &mut PgConnection, name: &str, credits: i32) -> Class {
    use crate::schema::class;    

    let new_class = NewClass { name, credits: &credits };

    diesel::insert_into(class::table)
        .values(&new_class)
        .get_result(conn)
        .expect("error saving new class")
    
}

fn create_class_component(conn: &mut PgConnection, class: Class) -> Component {
    use crate::schema::component;


    let new_class_comp = NewClassComponent { 
        name: &class.name,
        class: &class.id
    };

    diesel::insert_into(component::table)
        .values(&new_class_comp)
        .get_result(conn)
        .expect("Error creating class component")
}

fn push_classes(conn: &mut PgConnection, comp_classes: &mut Vec<Component>) {

    let classes = vec![("CNIT 15501", 3),
        ("CNIT 17600", 3),
        ("CNIT 18000", 3),
        ("CNIT 24200", 3),
        ("CNIT 25501", 3),
        ("CNIT 27000", 3),
        ("CNIT 27200", 3),
        ("CNIT 28000", 3),
        ("CNIT 32000", 3),
        ("CNIT 48000", 3),
        ("CNIT 37200", 3),
        ("CNIT 39200", 3),
        ("CNIT 31500", 3),
        ("CNIT 32500", 3),
        ("CNIT 38000", 3),
        ("CGT 25600", 3),
        ("CNIT 32200", 3),
        ("CNIT 24000", 3),
        ("CNIT 34500", 3),
        ("CNIT 34600", 3),
        ("CNIT 34400", 3),
        ("CNIT 34000", 3),
        ("CNIT 34010", 1),
        ("CNIT 34210", 1),
        ("CNIT 34220", 2),
        ("CNIT 37000", 3),
        ("CNIT 42000", 3),
        ("CNIT 42200", 3),
        ("CNIT 45500", 3),
        ("CNIT 47000", 3),
        ("CNIT 47100", 3),
        ("ITSEL 00000", 3),
        ("CSECSEL 00000", 3),
        ("SCLA 10100", 3),
        ("SCLA 10200", 3),
        ("TECH 12000", 3),
        ("MA 16010", 4),
        ("MA 16020", 4),
        ("OLS 25200", 3),
        ("TLI 11200", 3),
        ("COMSEL 00000", 3),
        ("ECONSEL 00000", 3),
        ("SCISEL 00000", 4),
        ("LABSCISEL 00000", 4),
        ("ACCSEL 00000", 3),
        ("STATSEL 00000", 3),
        ("SPEAKSEL 00000", 3),
        ("WRITINGSEL 00000", 3),
        ("INTERDISC 00000", 3),
        ("IET 10400", 3),
        ("IT 10400", 3),
        ("TLI 11100", 3),
        ("TLI 15200", 3),
        ("PHIL 15000", 3),
        ("HUMSEL 00000", 3),
        ("BEHAVSCISEL 00000", 3),
        ("FOUNDSEL 00000", 3),
        ("FREE 00000", 3),
        ("SAADSEL 00000", 3)
    ];

    for class in classes {

        //we make the classes in the class table
        let db_class = create_class(conn, class.0, class.1);

        //Then take this class and make a new component
        let class_component = create_class_component(conn, db_class);


    }

    //after that, we link those classes to the components.
}

fn push_components(conn: &mut PgConnection, components: &mut Vec<Component>) {
    use crate::schema::component;

    let component_strs = vec![
        "CNIT CORE",
        "CNIT DB PROGRAMMING",
        "CNIT SYS/APP DEV",
        "GENERAL BUSINESS SELECTIVE",
        "UNIV CORE",
        "CNIT/SAAD INTERDISC",
        "CSEC INTERDISC",
        "NENT INTERDISC",
        "CNIT IT SELECTIVES",
        "NENT IT SELECTIVES",
        "SAAD IT SELECTIVES",
        "CSEC SELECTIVES",
        "SAAD SELECTIVES"
    ];

    /*

    for component in component_strs {
        insert_into(component::table)
            .values((component::name.eq(component), component::pftype.eq("logical")))
            .execute(conn)
            .unwrap();
    }

    */

}

