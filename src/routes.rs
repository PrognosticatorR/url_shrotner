use actix_web::web;

pub mod urls;
use urls::{create_url, delete_url, get_redirect_url, get_urls, update_url};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_urls);
    cfg.service(get_redirect_url);
    cfg.service(delete_url);
    cfg.service(update_url);
    cfg.service(create_url);
}
