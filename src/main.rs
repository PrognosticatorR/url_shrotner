use actix_web::{middleware::Logger, web, App, HttpServer};
extern crate diesel;
use dotenv::dotenv;
use env_logger::{self, Env};

mod auth_guard;
mod models;
mod repositories;
mod routes;
mod schema;
mod utilities;

fn config(cfg: &mut web::ServiceConfig) {
    routes::configure(cfg);
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_pool: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>> =
        utilities::get_connection_pool();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::scope("/api/v1.0").configure(config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
