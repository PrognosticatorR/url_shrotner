use crate::models::urls::*;
use crate::schema::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
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
    pub async fn create_url(conn: &mut PgConnection, new_url: NewUrl) -> QueryResult<Url> {
        let _ = diesel::insert_into(urls::table)
            .values(new_url)
            .execute(conn);
        let last_inserted_id = Self::last_insert_id(conn)?;
        urls::table.find(last_inserted_id).first(conn)
    }

    fn last_insert_id(conn: &mut PgConnection) -> QueryResult<i64> {
        urls::table
            .select(urls::id)
            .order(urls::id.desc())
            .first(conn)
    }
}
