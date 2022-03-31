use mongodb::bson::{doc, Document};
use std::vec::Vec;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mongodb::{Client,Collection,Cursor,error::Error};
use crate::db;
use db::{CLIENT,getCollection};
use futures::stream::{StreamExt, TryStreamExt,Collect};
use crate::models::{category::Category};
use std::result::Result;



pub async fn getCategories(client: web::Data<Client>) -> HttpResponse {
    let collection = getCollection::<Category>("categories",client).await;
    let mut cursor = collection.find(None,None).await;

    let mut results = Vec::<Category>::new();
    // while let Some(result) = cursor.next().await {
    //     match result {
    //         Ok(category) => {
    //             results.push(category);
    //         }
    //         _ => {
    //             return HttpResponse::InternalServerError().finish();
    //         }
    //     }
    // }
    // HttpResponse::Ok().json(results)
    HttpResponse::Ok().body("terst")

}

pub async fn addCategory(client:web::Data<Client>) -> impl Responder {
    let collection = getCollection::<Category>("categories",client).await;

    let categories = vec![
        Category {
            _id:None,
            name: "Category1".to_string(),
        },
        Category {
            _id:None,
            name: "Category2".to_string(),
        },
    ];

    collection.insert_many(categories, None).await.expect("category register failed...");

    HttpResponse::Ok().body("category register success!")

}