mod api;
mod db_connection;
mod models;

use api::product::product_config;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
        .service(web::scope("/product").configure(product_config))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
