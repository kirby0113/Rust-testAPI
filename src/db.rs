use mongodb::{Client,Collection,options::{ClientOptions}};
use lazy_static::lazy_static;
use std::env;
use async_once::AsyncOnce;

// TODO: possibly remove lazy static reference to simplify
lazy_static! {
    pub static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        let client = create_mongo_client().await.unwrap();
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
    let mongo_connection_string = get_connection_string();
    Client::with_uri_str(mongo_connection_string).await

}

pub async fn getCollection<T>(coll_name: &str) -> Collection<T> {
    CLIENT.get().await.database("Rust-testAPI").collection(coll_name)
}