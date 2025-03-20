use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::prelude::*;
use std::env;
use dotenvy::dotenv;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    dotenv().ok(); // Load environment variables
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database: {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .unwrap_or_else(|e| {
            eprintln!("❌ Database connection error: {:?}", e);
            panic!("Could not connect to the database.");
        });

    // 🔹 Explicitly disable prepared statements
    {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let _ = diesel::sql_query("SET plan_cache_mode = force_generic_plan").execute(&mut conn);
    }

    println!("✅ Database connection successful!");
    pool
}
