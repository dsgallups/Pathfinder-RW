pub mod universities;
pub mod subdivisions;
pub mod components;

use actix_web::{web, HttpResponse};
use crate::db_connection::{ PgPool, PgPooledConnection };

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
        .get()
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}