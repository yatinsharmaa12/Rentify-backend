use bcrypt::{hash, verify, DEFAULT_COST};

// Hash a plain test password
pub fn hash_password(plain: %str) -> Result<String, bcrypt::BcryptError> {
    hash(plain, DEFAULT_COST)
}

// verify a plaintext password against a hashed password

pub fn verify_password(plain: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(plain, hashed)
}