use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::university::{NewUniversity, University, UniversityList};

use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;

// This is calling the list method on ProductList and
// serializing it to a json response
pub async fn index(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    match pg_pool_handler(pool) {
        Ok(mut pg_pool) => {
            return HttpResponse::Ok().json(UniversityList::list(&mut pg_pool));
        }
        Err(e) => {
            return e;
        }
    }
}

pub async fn create(
    new_university: web::Json<NewUniversity>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            return e;
        }
    };

    match new_university.create(&mut pg_pool) {
        Ok(univ) => {
            return HttpResponse::Ok().json(univ);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    }
}

pub async fn show(id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            return e;
        }
    };

    match University::find(&id, &mut pg_pool) {
        Ok(univ) => {
            return HttpResponse::Ok().json(univ);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    }
}

pub async fn destroy(id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            return e;
        }
    };

    match University::destroy(&id, &mut pg_pool) {
        Ok(_) => {
            return HttpResponse::Ok().json(());
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    }
}

pub async fn update(
    id: web::Path<i32>,
    new_university: web::Json<NewUniversity>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let mut pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            println!("pgpoolerr");
            return e;
        }
    };

    match University::update(&id, &new_university, &mut pg_pool) {
        Ok(_) => {
            return HttpResponse::Ok().json(());
        }
        Err(e) => {
            println!("univesrity err");
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    }
}
