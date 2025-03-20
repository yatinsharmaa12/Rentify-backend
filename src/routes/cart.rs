use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{db::pool::DbPool, models::cart::{CartItem, NewCartItem}};
use crate::schema::cart::dsl::*;
use crate::middleware::auth::AuthenticatedUser;

// ✅ Add to Cart
pub async fn add_to_cart(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    item: web::Json<NewCartItem>,
) -> impl Responder {
    let mut conn = pool.get().expect("DB connection failed");

    let new_cart_item = NewCartItem {
        user_id: user.user_id,
        product_id: item.product_id,
        quantity: item.quantity,
    };

    let _ = diesel::insert_into(cart)
        .values(&new_cart_item)
        .execute(&mut conn);

    HttpResponse::Created().json("Added to cart")
}

// ✅ Get Cart Items
pub async fn get_cart(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
) -> impl Responder {
    let mut conn = pool.get().expect("DB connection failed");
    let result = cart
    .filter(user_id.eq(user.user_id))
    .select((
        id,
        user_id,
        product_id,
        quantity,
        added_at.nullable(), // ✅ Match schema.rs
    ))
    .load::<CartItem>(&mut *conn);


    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch cart"),
    }
}

// ✅ Remove from Cart
pub async fn remove_from_cart(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
    cart_item_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("DB connection failed");

    let _ = diesel::delete(cart.filter(id.eq(*cart_item_id)))
        .execute(&mut conn);

    HttpResponse::Ok().json("Item removed")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cart")
            .route("/add", web::post().to(add_to_cart))
            .route("/list", web::get().to(get_cart))
            .route("/remove/{id}", web::delete().to(remove_from_cart)),
    );
}