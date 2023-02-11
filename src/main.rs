pub mod schema;
pub mod db_connection;
pub mod models;
pub mod handlers;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/universities", web::get().to(handlers::universities::index))
            .route("/universities", web::post().to(handlers::universities::create))
            .route("/universities/{id}", web::get().to(handlers::universities::show))
            .route("/universities/{id}", web::delete().to(handlers::universities::destroy))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}