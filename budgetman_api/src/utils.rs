use std::convert::Infallible;

use {
    common::{
        err_resp,
        models::user::{UserIdent, UserRow},
        with_db,
    },
    sqlx::SqlitePool,
    warp::{
        http::{header, StatusCode},
        Filter, Rejection,
    },
};

pub fn require_login(
    db: SqlitePool,
) -> impl Filter<Extract = (UserRow,), Error = Rejection> + Clone {
    warp::header(header::AUTHORIZATION.as_str())
        .or(warp::cookie("access_token"))
        .unify()
        .map(Some)
        .or_else(|_| async { Ok::<_, Infallible>((None,)) })
        .and(with_db(db))
        .and_then(check_token)
}

pub async fn check_token(token: Option<String>, db: SqlitePool) -> Result<UserRow, Rejection> {
    if let Some(token) = token {
        let token = token.trim_start_matches("Bearer ");
        let user_id = common::auth::validate_jwt(token)
            .map_err(|e| err_resp(StatusCode::UNAUTHORIZED, e.to_string()))
            .map_err(warp::reject::custom)?
            .id;

        crud::fetch_user_from(&db, &UserIdent::Id(user_id))
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
