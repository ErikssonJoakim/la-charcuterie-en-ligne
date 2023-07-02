mod db_connection;
mod models;

use actix_web::{ App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
