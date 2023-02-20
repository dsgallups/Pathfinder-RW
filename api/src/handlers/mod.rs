pub mod catalog;
pub mod components;
pub mod degrees;
pub mod dev;
pub mod schedule;
pub mod subdivisions;
pub mod types;
pub mod universities;

use crate::db_connection::{PgPool, PgPooledConnection};
use actix_web::{web, HttpResponse};

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
