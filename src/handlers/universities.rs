use actix_web::{web, HttpRequest, HttpResponse };

use crate::models::university::{
    University,
    NewUniversity,
    UniversityList
};

// This is calling the list method on ProductList and 
// serializing it to a json response
pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(UniversityList::list())
}

pub async fn create(new_university: web::Json<NewUniversity>) -> HttpResponse {

    match new_university.create() {
        Ok(univ) => {
            return HttpResponse::Ok().json(univ);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    }

}

pub async fn show(id: web::Path<i32>) -> HttpResponse {
    match University::find(&id) {
        Ok(univ) => {
            return HttpResponse::Ok().json(univ);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    }
        
}

pub async fn destroy(id: web::Path<i32>) -> HttpResponse {
    match University::destroy(&id) {
        Ok(_) => {
            return HttpResponse::Ok().json(());
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    }
}