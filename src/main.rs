use actix_web::{App, HttpServer};
use std::sync::*;
use mongodb::{Client,options::{ClientOptions,ResolverConfig},bson::{doc,Document}};
use actix_rt;
mod routes;
use std::env;
use dotenvy::dotenv;

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
      let client = Arc::new(create_mongo_client().await.unwrap());
      println!("CLIENTを取得しました！");
    
  //APIサーバー用意
    HttpServer::new(move || App::new().app_data(client.clone()).configure(routes::routes))
        .bind("localhost:8000")?
        .run()
        .await

}
