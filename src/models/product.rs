use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::products;
use chrono::{NaiveDateTime};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub image_url: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProductDB {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub image_url: String,
    pub created_at: Option<NaiveDateTime>,
}
