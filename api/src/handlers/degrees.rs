use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::degree::{Degree, DegreeList, NewDegree};

use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;

// This is calling the list method on ProductList and
// serializing it to a json response
pub async fn index(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    match pg_pool_handler(pool) {
        Ok(mut pg_pool) => HttpResponse::Ok().json(DegreeList::list(&mut pg_pool)),
        Err(e) => e,
    }
}

pub async fn create(new_degree: web::Json<NewDegree>, pool: web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => return e,
    };

    match new_degree.create(&mut pg_pool) {
        Ok(univ) => HttpResponse::Ok().json(univ),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn show(id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            return e;
        }
    };

    match Degree::find(&id, &mut pg_pool) {
        Ok(univ) => HttpResponse::Ok().json(univ),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
pub async fn show_code(code: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            return e;
        }
    };

    match Degree::find_by_code(&code, &mut pg_pool) {
        Ok(univ) => HttpResponse::Ok().json(univ),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
pub async fn destroy(id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            return e;
        }
    };

    match Degree::destroy(&id, &mut pg_pool) {
        Ok(_) => HttpResponse::Ok().json(()),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn update(
    id: web::Path<i32>,
    new_degree: web::Json<NewDegree>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            println!("pgpoolerr");
            return e;
        }
    };

    match Degree::update(&id, &new_degree, &mut pg_pool) {
        Ok(_) => HttpResponse::Ok().json(()),
        Err(e) => {
            println!("univesrity err");
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}
