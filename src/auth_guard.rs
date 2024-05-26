use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use actix_web::Error;
use actix_web::FromRequest;
use futures::future::{ready, Ready};
use jsonwebtoken::Validation;
use jsonwebtoken::{decode, Algorithm, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub email: String,
    pub id: i32,
    pub exp: usize,
}

#[derive(Debug)]
pub struct AuthGuard {
    pub claims: Claims,
}

impl FromRequest for AuthGuard {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header: Option<&HeaderValue> = req.headers().get("Authorization");
        let jwt_secret = std::env::var("JWT_SECRET").unwrap();
        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let key = DecodingKey::from_secret(jwt_secret.as_ref());
                    match decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)) {
                        Ok(token_data) => {
                            return ready(Ok(AuthGuard {
                                claims: token_data.claims,
                            }))
                        }
                        Err(_) => {
                            return ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))
                        }
                    }
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized(
            "No token provided",
        )))
    }
}
