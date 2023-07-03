mod get;
mod patch;

use get::get_product;
use patch::update_product_count;
use actix_web::web::ServiceConfig;

pub fn product_config(cfg: &mut ServiceConfig) {
    cfg
    .service(get_product)
    .service(update_product_count);
}