use diesel::RunQueryDsl;  // ✅ Ensure this is imported
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database: {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .unwrap_or_else(|e| panic!("Database connection error: {:?}", e));

    // ✅ Make `conn` mutable
    let mut conn = pool.get().expect("Failed to get connection from pool");
    if diesel::sql_query("SELECT 1").execute(&mut conn).is_ok() {
        println!("Database connection successful!");
    } else {
        panic!("Database connection test failed!");
    }

    pool
}
