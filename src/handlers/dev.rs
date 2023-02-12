use actix_web::{
    web, 
    HttpRequest, 
    HttpResponse 
};
use serde_json::json;

use crate::{models::{
        university::{
            University,
            NewUniversity,
            UniversityList
        },
        component::{
            Component,
            NewComponent,
            ComponentList
        }
}, db_connection::PgPooledConnection};

use crate::db_connection::{ PgPool };
use diesel::PgConnection;
use crate::handlers::pg_pool_handler;
use std::str;

use std::process::{Command, Output};

pub async fn reset_and_pop_db(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {

    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => {p}
        Err(e) => {
            return e;
        }
    };

    let reset_output = reset_all_tables();
    println!("-------------------------------------\nTables Reset! Output:\n\n{}\n-------------------------------------", str::from_utf8(&reset_output.stdout).unwrap());


    let mut classes: Vec<Component> = Vec::new();
    let mut components: Vec<Component> = Vec::new();

    push_classes(&mut pg_pool, &mut classes);
    push_components(&mut pg_pool, &mut classes);



    return HttpResponse::Ok().json(json!({"name": "hi"}));
}

fn reset_all_tables() -> Output {
    Command::new("diesel")
        .arg("database")
        .arg("reset")
        .output()
        .expect("Failed to reset tables. (diesel/src/dev.rs)")
}

fn push_classes(conn: &mut PgConnection, class_components: &mut Vec<Component>) {

}

fn push_components(conn: &mut PgPooledConnection, components: &mut Vec<Component>) {

}