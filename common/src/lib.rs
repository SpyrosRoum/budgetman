//! Functions, structs, and enums that are used across two or more packages

pub mod auth;
pub mod models;
pub mod requests;
pub mod responses;

use std::convert::Infallible;

use {
    jwt_simple::prelude::HS256Key,
    once_cell::sync::OnceCell,
    sqlx::SqlitePool,
    warp::{http::StatusCode, Filter},
};

use crate::responses::ErrorResponse;

static KEY: OnceCell<HS256Key> = OnceCell::new();

/// Set the secrete key used for JWTs
///
/// # Panic
/// Will panic if it gets called more than once
pub fn set_secret(secret: &str) {
    let key = HS256Key::from_bytes(secret.as_bytes());
    KEY.set(key).expect("KEY has been set before")
}

/// Get the secrete key used for JWTs
///
/// # Panic
/// Will panic if the key hasn't been set
pub fn get_secret() -> &'static HS256Key {
    KEY.get().expect("KEY has not been set")
}

/// Helper function to create [`ErrorResponse`]s on te fly,
/// Basically sorter version of [`ErrorResponse::new`]
pub fn err_resp<S: Into<String>>(code: StatusCode, msg: S) -> ErrorResponse {
    ErrorResponse::new(code, msg.into())
}

pub fn with_db(db: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
