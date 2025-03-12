use serde::{Serialize};
use diesel::prelude::*;
use crate::schema::users;
use diesel::{Insertable, Queryable};

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub username: String
}

#[derive(Insertable, Serialize,)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}