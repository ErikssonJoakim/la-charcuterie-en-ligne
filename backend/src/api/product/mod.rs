mod get;

use get::get_product;
use actix_web::web::ServiceConfig;

pub fn product_config(cfg: &mut ServiceConfig) {
    cfg.service(get_product);
}