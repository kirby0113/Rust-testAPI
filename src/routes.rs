use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::controllers::{index, comment};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.route("/comment", web::get().to(comment::getComments));
    cfg.route("/comment/{id}", web::get().to(comment::getComment));
    cfg.route("/comment",web::post().to(comment::postComment));
}