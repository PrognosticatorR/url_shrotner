use crate::{
    models::users::FormUser,
    models::users::{LoginUser, NewUser},
    repositories::users::UsersRepository,
    utilities::Pool,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[post("/users/login")]
pub async fn login(pool: web::Data<Pool>, login_info: web::Json<LoginUser>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let res = UsersRepository::authenticate_user(&mut conn, login_info.into_inner()).await;
    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            let error_message = format!("Error during login: {:?}", e);
            HttpResponse::Unauthorized().json(error_message)
        }
    }
}

#[post("/users/signup")]
pub async fn signup(pool: web::Data<Pool>, signup_info: web::Json<NewUser>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let result = UsersRepository::signup(&mut conn, signup_info.into_inner()).await;
    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            let error_message = format!("Error during signup: {:?}", e);
            HttpResponse::InternalServerError().json(error_message)
        }
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user(pool: web::Data<Pool>, user_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let result = UsersRepository::delete_user(&mut conn, user_id.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::Accepted().json("user deleted successfully!"),
        Err(e) => {
            let error_message = format!("Error during deleting user: {:?}", e);
            HttpResponse::BadRequest().json(error_message)
        }
    }
}

#[get("/users/{user_id}")]
pub async fn get_user_details(pool: web::Data<Pool>, user_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let result = UsersRepository::get_user(&mut conn, user_id.into_inner()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            let error_message = format!("Error during get user: {:?}", e);
            HttpResponse::NotFound().json(error_message)
        }
    }
}

#[put("/users/{user_id}")]
pub async fn update_user(
    pool: web::Data<Pool>,
    user_id: web::Path<i32>,
    update: web::Json<FormUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let result =
        UsersRepository::update(&mut conn, user_id.into_inner(), update.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("user deleted successfully!"),
        Err(e) => {
            let error_message = format!("Error during updating user details: {:?}", e);
            HttpResponse::InternalServerError().json(error_message)
        }
    }
}
