use mongodb::bson::{doc, Document};
use std::vec::Vec;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mongodb::{Client,Collection,Cursor,error::Error};
use crate::db;
use db::CLIENT;
use futures::stream::{StreamExt, TryStreamExt,Collect};
use crate::models::{category::Category};
use std::result::Result;



pub async fn getCategories() -> impl Responder {
    let collection = CLIENT.get().await.database("Rust-testAPI").collection::<Category>("categories");
    let mut cursor = collection.find(None,None).await.unwrap();

    let mut results = Vec::<Category>::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(category) => {
                results.push(category);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)

}