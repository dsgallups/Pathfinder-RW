use actix_web::{
    web, 
    HttpRequest, 
    HttpResponse 
};
use crate::db_connection::PgPool;

use crate::models::subdivision::{
    NewSubdivision,
    SubdivisionList
};

use crate::handlers::pg_pool_handler;

pub async fn index(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    match pg_pool_handler(pool) {
        Ok(mut pg_pool) => {
            return HttpResponse::Ok().json(SubdivisionList::list(&mut pg_pool));
        }
        Err(e) => {
            return e;
        }
    }
}

pub async fn create(new_subdivision: web::Json<NewSubdivision>, pool: web::Data<PgPool>) -> HttpResponse {

    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => {p}
        Err(e) => {
            return e;
        }
    };

    match new_subdivision.create(&mut pg_pool) {
        Ok(subd) => {
            return HttpResponse::Ok().json(subd);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    }
}