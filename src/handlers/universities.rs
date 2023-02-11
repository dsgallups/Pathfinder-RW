use actix_web::{ HttpRequest, HttpResponse };

use crate::models::university::UniversityList;

// This is calling the list method on ProductList and 
// serializing it to a json response
pub async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(UniversityList::list())
}
