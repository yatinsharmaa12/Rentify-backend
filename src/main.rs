use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
mod db;
mod models;
mod routes;
mod schema;
mod utils;
mod middleware; // ✅ Import middleware module
use db::pool::init_pool;
use crate::routes::user_routes::{sign_up, login};
use crate::routes::product_routes::config as product_routes_config;
use crate::routes::cart::config as cart_routes_config;
 // ✅ Import AuthenticatedUser

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = init_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type", "Authorization", "X-User-Id"]) // ✅ Allow X-User-Id header
                    .max_age(3600),
            )
            .service(
                web::scope("/api")
                    .route("/signup", web::post().to(sign_up))
                    .route("/login", web::post().to(login))
                    .configure(product_routes_config)
                    .configure(cart_routes_config), // ✅ Configure cart routes
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
