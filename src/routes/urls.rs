use crate::utilities::Pool;
use crate::{models::urls, repositories::urls::UrlsRepository};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("/urls")]
pub async fn get_urls(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let urls = UrlsRepository::get_all(&mut conn).await.unwrap();
    HttpResponse::Ok().json(json!(urls))
}

#[post("/urls")]
pub async fn create_url() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[post("/urls/<id>")]
pub async fn update_url() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[post("/urls/<id>")]
pub async fn delete_url() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[get("/urls/<short_url>")]
pub async fn get_redirect_url() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}
