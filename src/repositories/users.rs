use crate::{schema::users, utilities};
use chrono::Utc;
use diesel::{
    result::Error, Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};
use log::debug;

use crate::models::users::*;
pub struct UsersRepository;

impl UsersRepository {
    pub async fn signup(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<User> {
        let user_data = User::sanitize_userinfo(new_user.clone());
        let res = diesel::insert_into(users::table)
            .values(user_data)
            .execute(conn);
        match res {
            Ok(_) => {
                let last_user = Self::last_insert_id(conn).await.unwrap();
                Self::get_user(conn, last_user).await
            }
            Err(e) => return Err(e),
        }
    }
    async fn last_insert_id(conn: &mut PgConnection) -> QueryResult<i32> {
        users::table
            .select(users::id)
            .order(users::id.desc())
            .first(conn)
    }

    pub async fn get_user(conn: &mut PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(conn)
    }

    pub async fn delete_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<usize> {
        let update = DeleteUser {
            deleted: true,
            deleted_at: Utc::now().naive_utc(),
        };
        conn.transaction(|db_conn| {
            let user = users::table.find(user_id);
            diesel::update(user).set(update).execute(db_conn)?;
            diesel::delete(user).execute(db_conn)
        })
    }
    pub async fn find_user_by_name(conn: &mut PgConnection, name: &str) -> QueryResult<User> {
        users::table.filter(users::username.eq(name)).first(conn)
    }
    pub async fn find_user_by_email(conn: &mut PgConnection, email_id: &str) -> QueryResult<User> {
        users::table.filter(users::email.eq(email_id)).first(conn)
    }

    pub async fn authenticate_user(conn: &mut PgConnection, login: LoginUser) -> QueryResult<User> {
        let user = match (login.username, login.email) {
            (Some(username), _) => Self::find_user_by_name(conn, &username).await?,
            (_, Some(email_id)) => Self::find_user_by_email(conn, &email_id).await?,
            _ => return Err(Error::NotFound),
        };
        debug!("{:?}", user);
        match utilities::verify_hashed_password(&login.password, user.password.as_str()) {
            true => Ok(user),
            false => Err(Error::QueryBuilderError("Error verifying password".into())),
        }
    }

    pub async fn update(conn: &mut PgConnection, id: i32, update: FormUser) -> QueryResult<usize> {
        diesel::update(users::table.find(id))
            .set(update)
            .execute(conn)
    }
}
