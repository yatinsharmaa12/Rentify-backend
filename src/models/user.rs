use serde::{Deserialize, Serialize};
use diesel::perlude::*;
use crate::schema::users;

#[derive(Debug, Serialize, Queryable]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub username: String
}

#[derive(Insertable, Serialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}