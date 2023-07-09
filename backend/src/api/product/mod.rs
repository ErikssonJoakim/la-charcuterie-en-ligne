mod get;
mod patch;

use actix_web::web::ServiceConfig;
use get::get_product;
use patch::update_product_count;

pub fn product_config(cfg: &mut ServiceConfig) {
    cfg.service(get_product).service(update_product_count);
}
