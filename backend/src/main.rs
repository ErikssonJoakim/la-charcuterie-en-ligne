mod api;
mod db_connection;
mod model;

use actix_web::{web, App, HttpServer};
use api::products::products_config;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let address = env::var("API_ADDRESS");
    let port = env::var("API_PORT");

    HttpServer::new(move || App::new().service(web::scope("/products").configure(products_config)))
        .bind(format!("{}:{}", address.unwrap(), port.unwrap()))?
        .run()
        .await
}
