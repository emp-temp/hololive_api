use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

pub mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let addr = env::var("ADDR").expect("ADDR is must be set");

    HttpServer::new(|| App::new().service(handler::health))
        .bind(addr)?
        .run()
        .await
}
