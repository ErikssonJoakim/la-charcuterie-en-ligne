mod get;

use actix_web::web::ServiceConfig;
use get::get_products;

pub fn products_config(cfg: &mut ServiceConfig) {
    cfg.service(get_products);
}
