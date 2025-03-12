use actix_web:: {web,HttpResponse, Responder};
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
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
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    
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

    // Insert into database using web::block
    let inserted_user = web::block(move || {
        diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_select()) // Ensure `as_select()` is properly implemented
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
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let email_clone = form.email.clone(); // Clone email before moving

    // Query the user by email
    let user_result = web::block(move || {
        users.filter(email.eq(email_clone)).first::<User>(&mut conn)
    })
    .await;

    match user_result {
        Ok(Ok(user)) => {
            if verify_password(&form.password, &user.password_hash).unwrap_or(false) {
                HttpResponse::Ok().body("Welcome back!")
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
