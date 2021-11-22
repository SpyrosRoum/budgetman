use {
    argon2::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        },
        Argon2,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn validate_password(hash: &str, password: &str) -> bool {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(hash).expect("Invalid argon2 encoded password");
    argon2
        .verify_password(password.as_bytes(), &hash)
        .map_or(false, |_| true)
}
