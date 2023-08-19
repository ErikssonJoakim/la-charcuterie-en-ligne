mod get;
mod patch;

use actix_web::web::ServiceConfig;
use get::{get_product, get_products};
use patch::update_product_count;

pub fn products_config(cfg: &mut ServiceConfig) {
    cfg.service(get_products)
        .service(get_product)
        .service(update_product_count);
}
