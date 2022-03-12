use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn getComments() -> impl Responder {
    HttpResponse::Ok().body("get CommentList controller.")
}

pub async fn getComment(req:HttpRequest) -> impl Responder {
    let id:u8 = req.match_info().get("id").unwrap().parse().unwrap();
    HttpResponse::Ok().body(format!("get Comment controller. id={0}",id))
}

pub async fn postComment() -> impl Responder {
    HttpResponse::Ok().body("Post Comment controller.")
}