use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{
    models::product::{NewProductDB, Product},
    db::pool::DbPool,
    schema::products,
};
use chrono::Utc;
use crate::schema::products::dsl::*;
use crate::middleware::auth::AuthenticatedUser;  // ✅ Import authentication middleware

#[derive(serde::Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub image_url: String,
}

pub async fn add_product(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    new_product: web::Json<NewProduct>,
) -> impl Responder {
    // Obtain a database connection from the pool
    let conn = &mut pool.get().expect("Failed to get Db connection");

    // Force generic query plans to avoid prepared statement issues with NeonDB
    let _ = diesel::sql_query("SET plan_cache_mode = force_generic_plan")
        .execute(&mut *conn);

    let new_product_db = NewProductDB {
        user_id: user.user_id,
        name: new_product.name.clone(),
        description: new_product.description.clone(),
        price: new_product.price,
        image_url: new_product.image_url.clone(),
        created_at: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(products::table)
        .values(&new_product_db)
        .execute(conn)
        .expect("Failed to insert new product");

    HttpResponse::Created().json("Product added successfully")
}

pub async fn get_products(pool: web::Data<DbPool>) -> impl Responder {
    // Obtain a database connection from the pool
    let conn = &mut pool.get().expect("Failed to get Db connection");

    // Force generic query plans to avoid prepared statement issues with NeonDB
    let _ = diesel::sql_query("SET plan_cache_mode = force_generic_plan")
        .execute(&mut *conn);

    let result = crate::schema::products::table
        .select(Product::as_select())
        .load::<Product>(&mut *conn);

    match result {
        Ok(product_list) => HttpResponse::Ok().json(product_list),
        Err(e) => {
            eprintln!("Error fetching products: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch products")
        }
    }
}

pub async fn delete_product(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    product_id: web::Path<i32>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let target_product = products
        .filter(id.eq(*product_id))
        .first::<crate::models::product::Product>(conn)
        .optional();

    match target_product {
        Ok(Some(product)) => {
            if product.user_id != user.user_id {
                return HttpResponse::Forbidden().body("You are not allowed to delete this product");
            }

            diesel::delete(products.filter(id.eq(product.id)))
                .execute(conn)
                .expect("Failed to delete product");

            HttpResponse::Ok().json("Product deleted successfully")
        }
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .route("/add", web::post().to(add_product))
            .route("/list", web::get().to(get_products))
            .route("/delete/{id}", web::delete().to(delete_product)),
    );
}
