use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,   // User ID
    exp: usize, // Expiry timestamp
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Check for Authorization header
        let auth_header = match req.headers().get("Authorization") {
            Some(h) => h.to_str().unwrap_or(""),
            None => return ready(Err(actix_web::error::ErrorUnauthorized("Missing Authorization header"))),
        };

        // Ensure it's a Bearer token
        if !auth_header.starts_with("Bearer ") {
            return ready(Err(actix_web::error::ErrorUnauthorized("Invalid token format")));
        }
        let token = &auth_header[7..]; // Remove "Bearer " prefix

        // Decode JWT token
        let decoding_key = DecodingKey::from_secret("your_secret_key".as_ref()); // Replace with your secret
        match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)) {
            Ok(token_data) => {
                let user_id = token_data.claims.sub;
                ready(Ok(AuthenticatedUser { user_id }))
            }
            Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid or expired token"))),
        }
    }
}
