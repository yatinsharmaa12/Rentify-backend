use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    dotenv().ok(); // Load .env

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database: {}", database_url); // Debug log

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    match Pool::builder().max_size(10).build(manager) {
        Ok(pool) => {
            println!("Database connection pool created successfully!");
            pool
        }
        Err(e) => {
            eprintln!("Failed to create pool: {:?}", e);
            panic!("Database connection error: {:?}", e);
        }
    }
}
