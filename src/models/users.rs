use crate::{schema::users, utilities};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub deleted: bool,
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name=users)]
pub struct FormUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Insertable, Clone, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, AsChangeset)]
#[diesel(table_name=users)]
pub struct DeleteUser {
    pub deleted_at: NaiveDateTime,
    pub deleted: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

impl User {
    pub fn sanitize_userinfo(user_data: NewUser) -> NewUser {
        let hashed_password = utilities::get_password_hash(&user_data.password);
        NewUser {
            username: user_data.username,
            password: hashed_password,
            email: user_data.email,
        }
    }
}
