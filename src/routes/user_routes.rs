use actix_web:: {web,HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use crate::utils::{hash_password, verify_password};
use crate::schema::users::dsl::*;
use crate::db::pool::DbPool;


// Data Structure fo receiving signup request
#[derive(serde::Deserialize)]

pub struct SignUpData {
    pub email: String,
    pub password: String,
}

// Handler for SIGNUP 
pub async fn sign_up(
    pool: web::Data<DbPool>,
    form: web::Json<SignUpData>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    
    // Hash the password
    let hashed = match hash_password(&form.password) {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let new_user = NewUser {
        email: form.email.clone(),
        password_hash: hashed,
    };

    // Insert the new user into the database using diesel

    let inserted_user = web::block(move || {
        diesel::insert_into(users).values(&new_user).get_result::<User>(&mut *conn)
    }).await;

    match inserted_user {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) =>{
            eprintln!("Error: {:?}", e);
         HttpResponse::InternalServerError().json(e.to_string())
        }
    }

}


// Data structure for receiving login request


#[derive(serde::Deserialize)]

pub struct LoginData {
    pub email: String,
    pub password: String,
}

// Handler for user login
pub async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<LoginData>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // Query the user by email
    let user_result = web::block(move || {
        users.filter(email.eq(&form.email)).first::<User>(&mut conn)
    })
    .await;

    match user_result {
        Ok(user) => {
            match verify_password(&form.password, &user.password_hash) {
                Ok(true) => HttpResponse::Ok().body("Welcome back!"),
                Ok(false) => HttpResponse::Unauthorized().body("Invalid email or password"),
                Err(_) => HttpResponse::InternalServerError().body("Error verifying password"),
            }
        }
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::Unauthorized().body("Invalid email or password")
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Database error")
        }
    }
    
}