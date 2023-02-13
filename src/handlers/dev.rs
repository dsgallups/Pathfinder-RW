use actix_web::{
    web, 
    HttpRequest, 
    HttpResponse,
};
use serde_json::json;
use std::str;
use std::process::{Command, Output};


use crate::db_connection::{ PgPool };
use crate::handlers::{
    pg_pool_handler,
    catalog_maker::CatalogMaker
};




pub async fn reset_and_pop_db(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {

    let pg_pool = match pg_pool_handler(pool) {
        Ok(p) => {p}
        Err(e) => {
            return e;
        }
    };

    let reset_output = reset_all_tables();
    println!("-------------------------------------\nTables Reset! Output:\n\n{}\n-------------------------------------", str::from_utf8(&reset_output.stdout).unwrap());

    let mut c = CatalogMaker::new(pg_pool);
    
    c.gen_catalog();

    return HttpResponse::Ok().json(json!({"name": "hi"}));
}

fn reset_all_tables() -> Output {
    Command::new("diesel")
        .arg("database")
        .arg("reset")
        .output()
        .expect("Failed to reset tables. (diesel/src/dev.rs)")
}



