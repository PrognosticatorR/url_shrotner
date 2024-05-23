#![allow(unused)]

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
extern crate diesel;
use dotenv::dotenv;

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::scope("/api/v1.0").configure(config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
