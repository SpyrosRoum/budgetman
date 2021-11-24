use {
    argon2::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        },
        Argon2,
    },
    serde::{Deserialize, Serialize},
    warp::http::StatusCode,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
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

#[derive(Serialize, Debug, Clone)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl warp::reject::Reject for ErrorResponse {}

impl ErrorResponse {
    pub fn new<S: Into<String>>(code: StatusCode, msg: S) -> Self {
        Self {
            code: code.as_u16(),
            message: msg.into(),
        }
    }

    pub fn get_code(&self) -> u16 {
        self.code
    }
}
