use crate::models::{urls::Url, users::User};
use crate::schema::user_url_mappings;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Associations, Queryable, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Url))]
#[diesel(table_name=user_url_mappings)]

pub struct UserUrl {
    pub id: i64,
    pub user_id: i32,
    pub url_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name=user_url_mappings)]
pub struct NewUserUrls {
    pub user_id: i32,
    pub url_id: i64,
}
