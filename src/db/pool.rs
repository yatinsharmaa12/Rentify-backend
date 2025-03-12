use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::val("DATABASE_URL")
    .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}