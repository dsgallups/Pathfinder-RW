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

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(establish_connection()))
            .service(hello)
            .service(echo)
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
            .route("/schedule/{id}", web::get().to(handlers::dev::get_schedule))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
