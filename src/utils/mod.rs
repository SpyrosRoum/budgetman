pub(crate) mod auth;

use {jwt_simple::prelude::HS256Key, once_cell::sync::OnceCell};

static KEY: OnceCell<HS256Key> = OnceCell::new();

/// Set the secrete key used for JWTs
///
/// # Panic
/// Will panic if it gets called more than once
pub(crate) fn set_secret(secret: &str) {
    let key = HS256Key::from_bytes(secret.as_bytes());
    KEY.set(key).expect("KEY has been set before")
}

/// Get the secrete key used for JWTs
///
/// # Panic
/// Will panic if the key hasn't been set
pub(crate) fn get_secret() -> &'static HS256Key {
    KEY.get().expect("KEY has not been set")
}

pub(crate) fn err_is_failed_constraint(err: &sqlx::Error) -> bool {
    err.as_database_error()
        .map(|e| e.message().starts_with("UNIQUE"))
        .unwrap_or(false)
}
