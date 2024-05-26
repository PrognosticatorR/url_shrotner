use crate::models::{urls::*, user_url_mappings::NewUserUrls};
use crate::schema::*;
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use log::info;
pub struct UrlsRepository;

impl UrlsRepository {
    pub async fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<Url>> {
        urls::table.limit(100).get_results(conn)
    }
    pub async fn get_url(conn: &mut PgConnection, url_query: &str) -> QueryResult<Url> {
        urls::table
            .filter(urls::short_url.eq(url_query))
            .get_result(conn)
    }
    pub async fn update_url(
        conn: &mut PgConnection,
        id: i64,
        update: FormUrl,
    ) -> QueryResult<usize> {
        diesel::update(urls::table.find(id))
            .set(update)
            .execute(conn)
    }
    pub async fn delete_url(conn: &mut PgConnection, id: i64) -> QueryResult<usize> {
        diesel::delete(urls::table.find(id)).execute(conn)
    }

    pub async fn find_by_id(conn: &mut PgConnection, id: i64) -> QueryResult<Url> {
        info!("findind url for id: {}", id);
        urls::table.find(id).get_result::<Url>(conn)
    }
    pub async fn create_url(
        conn: &mut PgConnection,
        new_url: NewUrl,
        user_id: i32,
    ) -> QueryResult<Url> {
        conn.transaction(|db_conn| {
            let url = diesel::insert_into(urls::table)
                .values(new_url)
                .get_result::<Url>(db_conn);

            match url {
                Ok(inserted_url) => {
                    let url_mapping = NewUserUrls {
                        user_id,
                        url_id: inserted_url.id,
                    };

                    diesel::insert_into(user_url_mappings::table)
                        .values(url_mapping)
                        .execute(db_conn)?;

                    Ok(inserted_url)
                }
                Err(e) => Err(e),
            }
        })
    }
}
