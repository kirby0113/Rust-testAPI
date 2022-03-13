use actix_web::{App, HttpServer};
use mongodb::{Client,options::{ClientOptions,ResolverConfig},bson::{doc,Document}};
use tokio;
use actix_rt;
use Rust_testAPI::routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    //MongoDB用意

    let client_options = ClientOptions::parse(
        "mongodb+srv://0113ryo2000:0113ryo2000@rust-testapi.kxdjy.mongodb.net/Rust_testAPI?retryWrites=true&w=majority",
    )
    .await.expect("Database Link Error.");

// Get a handle to the deployment.
let client = Client::with_options(client_options).expect("Some error message");

let database = client.database("Rust-testAPI");

let collection = database.collection::<Document>("Test");
  //APIサーバー用意
    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("localhost:8000")?
        .run()
        .await

}
