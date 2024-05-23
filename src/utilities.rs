use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("NO DATABASE_URL FOUND!");
    println!("{}", database_url);
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection Pool")
}
