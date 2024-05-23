use crate::models::urls::*;
use crate::schema::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
pub struct UrlsRepository;

impl UrlsRepository {
    pub async fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<Url>> {
        urls::table.limit(100).get_results(conn)
    }
    pub async fn get_url() -> QueryResult<Url> {
        todo!()
    }
    pub async fn update_url() -> QueryResult<usize> {
        todo!()
    }
    pub async fn delete_url() -> QueryResult<usize> {
        todo!()
    }
    pub async fn create_url(conn: &mut PgConnection, new_url: NewUrl) -> QueryResult<Url> {
        diesel::insert_into(urls::table)
            .values(new_url)
            .execute(conn);
        let last_inserted_id = Self::last_insert_id(conn)?;
        urls::table.find(last_inserted_id).get_result(conn)
    }

    fn last_insert_id(conn: &mut PgConnection) -> QueryResult<i64> {
        urls::table
            .select(urls::id)
            .order(urls::id.desc())
            .first(conn)
    }
}
