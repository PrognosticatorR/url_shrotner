use actix_web::web;

pub mod urls;
pub mod users;
use urls::{create_url, delete_url, get_redirect_url, get_url, get_urls, update_url};
use users::{delete_user, get_user_details, login, signup, update_user};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_urls);
    cfg.service(get_redirect_url);
    cfg.service(delete_url);
    cfg.service(update_url);
    cfg.service(create_url);
    cfg.service(get_url);
    cfg.service(delete_user);
    cfg.service(get_user_details);
    cfg.service(login);
    cfg.service(signup);
    cfg.service(update_user);
}
