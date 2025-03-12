use actix_web::{web, App, HttpServer};
mod db;
mod models;
mod routes;
mod schema;
mod utils;

use db::pool::{establish_connection_pool, DbPool};

use routes::user_routes::{sign_up, login};

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    // Create the database connection pool

    let pool: DbPool = establish_connection_pool();

    HttpServer::new(mode || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .services(
            web::scope("/api")
            .route("/signup", web::post().to(sign_up))
            .route("/login", web::post().to(login))
        )
    })
    .bind("127.0.0.1", 8080)?
    .run()
    .await
}
