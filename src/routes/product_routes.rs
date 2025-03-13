use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{models::product::Product, db::pool::DbPool, schema::products};
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub image_url: String,
}


pub async fn add_product(
      pool: web::Data<DbPool>,
      user_id: web::ReqData<i32>,
      new_product: web::Json<NewProduct>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get Db connection");

    let new_product = Product {
        id : 0,
        user_id: user_id.into_inner(),
        name: new_product.name.clone(),
        description: new_product.description.clone(),
        price: new_product.price,
        image_url: new_product.image_url.clone(),
        created_at: Some(chrono::Utc::now().naive_utc()), 
    };

    diesel::insert_into(products::table)
    .values(&new_product)
    .execute(conn)
    .expect("Failed to insert new product");

    HttpResponse::Created().json("Product added successfully")
}

