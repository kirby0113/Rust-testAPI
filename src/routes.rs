use actix_web::{web, HttpRequest, HttpResponse, Responder};
use Rust_testAPI::controllers::{index, comment,category};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.route("/comment", web::get().to(comment::getComments));
    cfg.route("/comment/{id}", web::get().to(comment::getComment));
    cfg.route("/comment",web::post().to(comment::postComment));
    cfg.route("/categories",web::get().to(category::getCategories));
    cfg.route("/categories/register",web::get().to(category::addCategory));
}