mod api;
mod db_connection;
mod model;

use api::{product::product_config, products::products_config};
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
        .service(web::scope("/product").configure(product_config))
        .service(web::scope("/products").configure(products_config))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
