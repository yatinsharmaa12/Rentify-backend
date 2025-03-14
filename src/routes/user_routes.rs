use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use serde_json::json;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use crate::models::user::{User, NewUser};
use crate::utils::{hash_password, verify_password};
use crate::schema::users::dsl::*;
use crate::db::pool::DbPool;

// Data Structure for receiving signup request
#[derive(serde::Deserialize)]
pub struct SignUpData {
    pub email: String,
    pub password: String,
}

// JWT Token Claims
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32,  // User ID
    exp: usize, // Expiration timestamp
}

// Function to generate JWT token
fn generate_jwt(user_id: i32) -> String {
    let claims = Claims {
        sub: user_id,
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"your_secret_key"),  // Change this secret key
    )
    .expect("Failed to generate token")
}

// 🔹 SIGNUP HANDLER
pub async fn sign_up(
    pool: web::Data<DbPool>,
    form: web::Json<SignUpData>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    // Hash the password
    let hashed = match hash_password(&form.password) {
        Ok(v) => v,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let new_user = NewUser {
        user_handle: form.email.clone(),
        name: "New User".to_string(),
        email: form.email.clone(),
        password_hash: hashed,
    };

    // Insert into database
    let inserted_user = web::block(move || {
        diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_select())
            .get_result::<User>(&mut conn)
    })
    .await;

    match inserted_user {
        Ok(Ok(user)) => HttpResponse::Created().json(user),
        Ok(Err(e)) => {
            eprintln!("Diesel error: {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
        Err(e) => {
            eprintln!("BlockingError: {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

// 🔹 LOGIN HANDLER
#[derive(serde::Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

pub async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<LoginData>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");
    let email_clone = form.email.clone();

    // Query the user by email
    let user_result = web::block(move || {
        users.filter(email.eq(email_clone)).first::<User>(&mut conn)
    })
    .await;

    match user_result {
        Ok(Ok(user)) => {
            if verify_password(&form.password, &user.password_hash).unwrap_or(false) {
                let token = generate_jwt(user.id);
                HttpResponse::Ok().json(json!({ "token": token }))
            } else {
                HttpResponse::Unauthorized().body("Invalid email or password")
            }
        }
        Ok(Err(diesel::result::Error::NotFound)) => {
            HttpResponse::Unauthorized().body("Invalid email or password")
        }
        _ => {
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}
