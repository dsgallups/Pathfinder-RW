use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use std::process::{Command, Output};
use std::str;

use crate::db_connection::PgPool;
use crate::handlers::{catalog::Catalog, pg_pool_handler, schedule::ScheduleMaker};

pub async fn reset_and_pop_db(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            return e;
        }
    };

    let reset_output = reset_all_tables();
    println!("-------------------------------------\nTables Reset! Output:\n\n{}\n-------------------------------------", str::from_utf8(&reset_output.stdout).unwrap());

    let mut c = Catalog::new(pg_pool);

    c.gen_test_catalog();

    HttpResponse::Ok().json(json!({"result": "success"}))
}

pub async fn get_schedule(degree_name: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    println!("GET Request to get_schedule/");
    let pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            return e;
        }
    };

    let mut schedule =
        ScheduleMaker::new(pg_pool, &degree_name).expect("Schedule failed to build!");

    schedule.build_schedule().expect("Failed");

    HttpResponse::Ok().json(json!({"name": "hi"}))
}

fn reset_all_tables() -> Output {
    Command::new("diesel")
        .arg("database")
        .arg("reset")
        .output()
        .expect("Failed to reset tables. (diesel/src/dev.rs)")
}
