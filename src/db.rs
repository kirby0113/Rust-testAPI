use mongodb::{Client,Collection,options::{ClientOptions}};
use actix_web::web;
use lazy_static::lazy_static;
use std::env;
use async_once::AsyncOnce;

// TODO: possibly remove lazy static reference to simplify
lazy_static! {
    pub static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        let client = create_mongo_client().await.unwrap();
        println!("CLIENT実行。");
        client
    });
}


fn get_connection_string() -> String {
    let host = env::var("MONGO_HOST").expect("MONGO_HOST env not set."); 
    let user = env::var("MONGO_USER").expect("MONGO_USER env not set."); 
    let password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env not set."); 
    let query = env::var("MONGO_QUERY").expect("MONGO_QUERY env not set."); 
    "mongodb+srv://".to_owned() + &user + ":" + &password + "@" + &host + "?" + &query
}

async fn create_mongo_client() -> Result<Client,mongodb::error::Error> {
    let client_options = ClientOptions::parse(
        get_connection_string(),
    ).await?;
    let client = Client::with_options(client_options);
    client
}

pub async fn getCollection<T>(coll_name: &str,client:web::Data<Client>) -> Collection<T> {
    CLIENT.get().await.database("Rust-testAPI").collection(coll_name)
}