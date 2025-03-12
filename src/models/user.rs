use diesel::prelude::*;
use diesel::Queryable;
use diesel::Insertable;
use serde::Serialize;
use crate::schema::users;


#[derive(Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub user_handle: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_handle: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}