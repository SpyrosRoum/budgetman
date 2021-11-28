use {
    argon2::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        },
        Argon2,
    },
    jwt_simple::prelude::*,
};

use crate::{
    get_secret,
    models::user::{self, UserClaims},
};

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

pub fn create_jwt(user: &user::UserRow) -> Result<String, jwt_simple::Error> {
    let key = get_secret();
    let claims = UserClaims {
        id: user.id.to_owned(),
    };
    let claims = Claims::with_custom_claims(claims, Duration::from_hours(2));
    key.authenticate(claims)
}

pub fn validate_jwt(token: &str) -> Result<UserClaims, jwt_simple::Error> {
    let key = get_secret();
    let claims = key.verify_token::<UserClaims>(token, None)?;
    Ok(claims.custom)
}
