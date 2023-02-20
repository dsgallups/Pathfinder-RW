#![allow(dead_code, unused_variables)]
pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

//test

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use serde_json::json;
use std::process::{Command, Output};
use std::str;

use crate::handlers::{
    catalog::Catalog, pg_pool_handler, schedule::ScheduleMaker, types::ScheduleError,
};
use crate::{db_connection::PgPool, handlers::types::Schedule};

use db_connection::establish_connection;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[allow(dead_code)]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
//.route("/schedule/{id}", web::get().to(handlers::dev::get_schedule))
#[get("/schedule/{id}")]
pub async fn get_schedule(
    degree_name: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<web::Json<Schedule>, ScheduleError> {
    println!("GET Request to get_schedule/");
    let pg_pool = match pg_pool_handler(pool) {
        Ok(p) => p,
        Err(e) => {
            panic!("pool bad!")
        }
    };

    let mut schedule = ScheduleMaker::new(pg_pool, &degree_name)?;

    match schedule.build_schedule() {
        Ok(res) => {
            println!("built schedule");
            return Ok(web::Json(res));
        }
        Err(e) => {
            println!("Error building schedule: {}", e);
            return Err(e);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(establish_connection()))
            .service(hello)
            .service(echo)
            .service(get_schedule)
            .route(
                "/universities",
                web::get().to(handlers::universities::index),
            )
            .route(
                "/universities",
                web::post().to(handlers::universities::create),
            )
            .route(
                "/universities/{id}",
                web::get().to(handlers::universities::show),
            )
            .route(
                "/universities/{id}",
                web::delete().to(handlers::universities::destroy),
            )
            .route(
                "/universities/{id}",
                web::patch().to(handlers::universities::update),
            )
            .route(
                "/subdivisions",
                web::post().to(handlers::subdivisions::create),
            )
            .route(
                "/subdivisions",
                web::get().to(handlers::universities::index),
            )
            .route("/components", web::get().to(handlers::components::index))
            .route(
                "/reset_and_pop_db",
                web::get().to(handlers::dev::reset_and_pop_db),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
