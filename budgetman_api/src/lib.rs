mod handlers;
pub mod utils;

use std::convert::Infallible;

use {
    common::{models::account::*, requests::*, responses::ErrorResponse},
    sqlx::SqlitePool,
    warp::{
        self, body::BodyDeserializeError, http::StatusCode, reject::InvalidQuery, Filter,
        Rejection, Reply,
    },
};

use crate::utils::require_login;

pub fn routes(db: &SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // Might want to play with GraphQL later or simply do breaking changes to the api, so we use `v1` path
    let base = warp::path!("api" / "v1" / ..);
    let base_logged_in = base
        .and(common::with_db(db.clone()))
        .and(require_login(db.clone()));

    let login = base
        .and(warp::path!("login"))
        .and(warp::post())
        .and(warp::body::json::<LoginRequest>())
        .and(common::with_db(db.clone()))
        .and_then(handlers::handle_login);

    let get_accounts = base_logged_in
        .clone()
        .and(warp::path!("accounts"))
        .and(warp::get())
        .and(warp::query::query::<AccountTypeQuery>().or_else(|_| async {
            // If there is no type specified, assume Any
            Ok::<_, Infallible>((AccountTypeQuery::new(AccountType::Any),))
        }))
        .and_then(handlers::get_accounts);

    let create_acc = base_logged_in
        .clone()
        .and(warp::path!("accounts"))
        .and(warp::post())
        .and(warp::body::json::<AccountCreateRequest>())
        .and_then(handlers::create_adhoc_account);

    let accounts = get_accounts.or(create_acc);

    login.or(accounts).recover(handle_rejection)
}

async fn handle_rejection(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(e) = r.find::<ErrorResponse>() {
        let code = StatusCode::from_u16(e.get_code()).expect("Constructed from StatusCode");
        let json = warp::reply::json(&e);
        Ok(warp::reply::with_status(json, code))
    } else if let Some(e) = r.find::<InvalidQuery>() {
        let code = StatusCode::BAD_REQUEST;
        let json = serde_json::json!({ "code": code.as_u16(), "message": e.to_string() });
        Ok(warp::reply::with_status(warp::reply::json(&json), code))
    } else if let Some(e) = r.find::<BodyDeserializeError>() {
        let code = StatusCode::BAD_REQUEST;
        let json = serde_json::json!({ "code": code.as_u16(), "message": e.to_string() });
        Ok(warp::reply::with_status(warp::reply::json(&json), code))
    } else {
        Err(r)
    }
}
