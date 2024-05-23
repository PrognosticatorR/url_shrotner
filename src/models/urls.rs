use crate::schema::urls;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable)]
#[diesel(table_name=urls)]

pub struct Url {
    pub id: i64,
    pub origin_url: String,
    pub short_url: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name=urls)]

pub struct NewUrl {
    origin_url: String,
    short_url: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name=urls)]
pub struct FormUrl {
    origin_url: String,
    short_url: String,
    updated_at: Option<NaiveDateTime>,
}
