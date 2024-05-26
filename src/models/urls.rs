use crate::schema::urls;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Clone, Identifiable)]

pub struct Url {
    pub id: i64,
    pub origin_url: String,
    pub short_url: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Clone)]
#[diesel(table_name=urls)]

pub struct NewUrl {
    pub origin_url: String,
    #[serde(skip_deserializing)]
    pub short_url: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name=urls)]
pub struct FormUrl {
    origin_url: Option<String>,
    short_url: Option<String>,
    updated_at: Option<NaiveDateTime>,
}

impl Url {
    pub fn from_url(url: &Url) -> Self {
        let app_host = std::env::var("APP_HOST").unwrap();
        Url {
            short_url: format!("{}/{}", app_host, url.short_url),
            id: url.id,
            origin_url: url.origin_url.clone(),
            updated_at: url.updated_at,
            created_at: url.created_at,
        }
    }
}
