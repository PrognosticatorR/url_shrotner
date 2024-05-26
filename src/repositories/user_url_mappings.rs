use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};

use crate::models::urls::Url;
use crate::schema::user_url_mappings;
use crate::schema::{urls, users};
pub struct UserUrlMappingRepository;

impl UserUrlMappingRepository {
    pub async fn get_all_urls_of_user(conn: &mut PgConnection, id: i32) -> QueryResult<Vec<Url>> {
        user_url_mappings::table
            .inner_join(users::table)
            .inner_join(urls::table)
            .filter(users::id.eq(id))
            .select((
                urls::id,
                urls::origin_url,
                urls::short_url,
                urls::created_at,
                urls::updated_at,
            ))
            .load::<Url>(conn)
    }
}
