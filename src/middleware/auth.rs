use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Extract user_id from request headers (if using JWT, decode token here)
        if let Some(user_id) = req.headers().get("X-User-Id") {
            if let Ok(user_id) = user_id.to_str().unwrap_or("0").parse::<i32>() {
                return ready(Ok(AuthenticatedUser { user_id }));
            }
        }

        // ❌ Unauthorized if no user_id
        ready(Err(actix_web::error::ErrorUnauthorized("User not authenticated")))
    }
}
