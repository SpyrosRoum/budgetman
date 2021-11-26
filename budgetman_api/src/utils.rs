use {
    sqlx::SqlitePool,
    common::{err_resp, models::user::{UserIdent, UserRow}, with_db},
    warp::{http::StatusCode, Rejection, Filter}
};

pub(crate) fn require_login(
    db: SqlitePool,
) -> impl Filter<Extract = (UserRow,), Error = Rejection> + Clone {
    warp::header::optional("authentication")
        .and(with_db(db))
        .and_then(check_token)
}

pub async fn check_token(token: Option<String>, db: SqlitePool) -> Result<UserRow, Rejection> {
    if let Some(token) = token {
        let user_id = common::auth::validate_jwt(token)
            .map_err(|e| err_resp(StatusCode::UNAUTHORIZED, e.to_string()))
            .map_err(warp::reject::custom)?
            .id;

        crud::get_user_from(&db, &UserIdent::Id(user_id))
            .await
            .map_err(|e| err_resp(StatusCode::UNAUTHORIZED, e.to_string()))?
            .ok_or_else(|| err_resp(StatusCode::BAD_REQUEST, "Invalid JWT"))
            .map_err(warp::reject::custom)
    } else {
        Err(warp::reject::custom(err_resp(
            StatusCode::BAD_REQUEST,
            "Authentication required",
        )))
    }
}
