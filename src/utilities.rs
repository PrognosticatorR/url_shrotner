#![allow(unused)]
use crate::{auth_guard::Claims, models::users::User};
use ::r2d2::PooledConnection;
use actix_web::{web, FromRequest, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use chrono::{Duration, Utc};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use hex;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sha2::{Digest, Sha256};

pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("NO DATABASE_URL FOUND!");
    println!("{}", database_url);
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection Pool")
}
pub fn get_password_hash(password: &str) -> String {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    PasswordHash::new(&hash).unwrap().to_string()
}

pub fn get_hash_for_string(string_to_hash: &str, length: usize) -> String {
    let rng = thread_rng();
    let random_bytes = rng
        .sample_iter(&Alphanumeric)
        .take(string_to_hash.len())
        .collect::<Vec<u8>>();
    let random_string = String::from_utf8_lossy(&random_bytes);
    let combined_string = format!("{}{}", string_to_hash, random_string);
    let mut hasher = Sha256::new();
    hasher.update(combined_string.as_bytes());
    let hash_result = hasher.finalize();
    let hash_string = hex::encode(&hash_result[..length]);
    hash_string
}

pub fn verify_hashed_password(password: &str, hashed_password: &str) -> bool {
    let hash = PasswordHash::new(hashed_password).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok()
}

pub fn generate_jwt_token(user_data: &User) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        id: user_data.id,
        exp: expiration, // Set expiration time
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();
    format!("Bearer {}", token)
}
