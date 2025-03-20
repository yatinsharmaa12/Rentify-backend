use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::cart;

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = cart)]
pub struct CartItem {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub added_at: Option<chrono::NaiveDateTime>, // ✅ Match schema.rs
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = cart)]
pub struct NewCartItem {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
