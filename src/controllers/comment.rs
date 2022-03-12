use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn comment() -> impl Responder {
    HttpResponse::Ok().body("Comment controller.")
}