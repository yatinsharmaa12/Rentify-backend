use actix_web::{web, App, HttpServer};
mod db;
mod models;
mod routes;
mod schema;
mod utils;

// use db::pool::{establish_connection_pool, DbPool};

use crate::routes::user_routes::{sign_up, login};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::pool::init_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                .route("/signup", web::post().to(sign_up))
                .route("/login", web::post().to(login))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
