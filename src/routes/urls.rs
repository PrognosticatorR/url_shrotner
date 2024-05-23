use crate::models::urls::{FormUrl, NewUrl, Url};
use crate::repositories::urls::UrlsRepository;
use crate::utilities::{self, Pool};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::debug;
use serde_json::json;

#[get("/urls")]
pub async fn get_urls(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let urls = UrlsRepository::get_all(&mut conn).await.unwrap();
    let modified_urls = urls
        .into_iter()
        .map(|url| Url::from_url(&url))
        .collect::<Vec<Url>>();
    HttpResponse::Ok().json(json!(modified_urls))
}

#[post("/urls")]
pub async fn create_url(pool: web::Data<Pool>, req_body: web::Json<NewUrl>) -> impl Responder {
    let hashed_string = utilities::get_hash_for_string(&req_body.origin_url, 18);
    let new_url = NewUrl {
        short_url: hashed_string,
        origin_url: req_body.origin_url.clone(),
    };
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let new_url = UrlsRepository::create_url(&mut conn, new_url)
        .await
        .unwrap();

    HttpResponse::Ok().json(json!(Url::from_url(&new_url)))
}

#[put("/urls/{id}")]
pub async fn update_url(
    pool: web::Data<Pool>,
    id: web::Path<i64>,
    update: web::Json<FormUrl>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    UrlsRepository::update_url(&mut conn, id.into_inner(), update.into_inner())
        .await
        .unwrap();
    HttpResponse::Ok().json(json!("url updated successfully!"))
}

#[delete("/urls/{id}")]
pub async fn delete_url(pool: web::Data<Pool>, id: web::Path<i64>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    UrlsRepository::delete_url(&mut conn, id.into_inner())
        .await
        .unwrap();
    HttpResponse::Ok().json(json!("url deleted successfully!"))
}

#[get("/{url}")]
pub async fn get_redirect_url(pool: web::Data<Pool>, url: web::Path<String>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    match UrlsRepository::get_url(&mut conn, &url.into_inner()).await {
        Ok(data) => {
            let redirect_url = data.origin_url;
            return HttpResponse::Found()
                .append_header(("Location", redirect_url))
                .finish();
        }
        Err(_) => return HttpResponse::NotFound().finish(),
    }
}

#[get("/urls/{id}")]
pub async fn get_url(pool: web::Data<Pool>, id: web::Path<i64>) -> impl Responder {
    debug!("Got a request for url id: {}", id);
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    match UrlsRepository::find_by_id(&mut conn, id.into_inner()).await {
        Ok(data) => {
            let redirect_url = data.origin_url;
            return HttpResponse::Found()
                .insert_header(("Location", redirect_url))
                .finish();
        }
        Err(_) => return HttpResponse::BadRequest().finish(),
    }
}
