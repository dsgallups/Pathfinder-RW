use actix_web::{
    web, 
    HttpRequest, 
    HttpResponse 
};
use serde_json::json;

use crate::models::university::{
    University,
    NewUniversity,
    UniversityList
};

use crate::db_connection::{ PgPool };
use crate::handlers::pg_pool_handler;


pub async fn reset_and_pop_db(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {

    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => {p}
        Err(e) => {
            return e;
        }
    };
    
    return HttpResponse::Ok().json(json!("{'name': 'hi'}"));
}