use crate::{
    auth_guard::AuthGuard,
    models::users::{FormUser, LoginUser, NewUser},
    repositories::{user_url_mappings::UserUrlMappingRepository, users::UsersRepository},
    utilities::{self, Pool},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[post("/users/login")]
pub async fn login(pool: web::Data<Pool>, login_info: web::Json<LoginUser>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json("Failed to get DB connection from pool")
        }
    };

    let res = UsersRepository::authenticate_user(&mut conn, login_info.into_inner()).await;
    match res {
        Ok(user) => {
            let jwt_token = utilities::generate_jwt_token(&user);
            let update = FormUser {
                token: Some(jwt_token.clone()), // Clone to reuse the token below
                username: Some(user.username.clone()),
                password: Some(user.password.clone()),
                email: Some(user.email.clone()),
            };
            if let Err(e) = UsersRepository::update(&mut conn, user.id, update).await {
                return HttpResponse::InternalServerError()
                    .json(format!("Error updating user: {:?}", e));
            }
            HttpResponse::Ok().json(json!( {"token":jwt_token})) // Return the JWT token
        }
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
pub async fn delete_user(
    pool: web::Data<Pool>,
    user_id: web::Path<i32>,
    _auth_info: AuthGuard,
) -> impl Responder {
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
pub async fn get_user_details(
    pool: web::Data<Pool>,
    user_id: web::Path<i32>,
    _auth_info: AuthGuard,
) -> impl Responder {
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
    _auth_info: AuthGuard,
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

#[get("/users/user/data")]
pub async fn get_info_for_user(pool: web::Data<Pool>, auth_info: AuthGuard) -> impl Responder {
    println!("{:?}", pool);
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json("Failed to get DB connection from pool")
        }
    };
    let result =
        UserUrlMappingRepository::get_all_urls_of_user(&mut conn, auth_info.claims.id).await;
    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            let err_message = format!("Error while fetchiond data for user: {}", e);
            HttpResponse::InternalServerError().json(err_message)
        }
    }
}
