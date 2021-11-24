use {sqlx::SqlitePool, warp::{http::StatusCode, Rejection, Reply}};

use crate::utils::error_response;

pub(crate) async fn handle_login(
    req: common::LoginRequest,
    db: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let user = crud::get_user_from_username(&db, req.username.as_str())
        .await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| error_response(StatusCode::UNAUTHORIZED, "Incorrect username or password"))?;

    if !common::validate_password(&user.password_hash, &req.password) {
        return Err(error_response(StatusCode::UNAUTHORIZED, "Incorrect username or password").into());
    }

    // construct and return a JWT instead of the user object
    Ok(warp::reply::json(&user))
}
