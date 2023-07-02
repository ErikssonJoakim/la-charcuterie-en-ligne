mod get;

use get::get_products;
use actix_web::web::ServiceConfig;

pub fn products_config(cfg: &mut ServiceConfig) {
    cfg.service(get_products);
}