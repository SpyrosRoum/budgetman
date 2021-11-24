mod handlers;
mod utils;

use std::convert::Infallible;

use {
    common::ErrorResponse,
    sqlx::SqlitePool,
    warp::{self, http::StatusCode, Filter, Rejection, Reply},
};

use utils::error_response;

pub fn routes(db: &SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // Might want to play with GraphQL later or simply do breaking changes to the api, so we use `v1` path
    let base = warp::path!("api" / "v1" / ..);

    let login = base
        .and(warp::path!("login"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::handle_login);

    login.recover(handle_rejection)
}

fn with_db(db: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn handle_rejection(e: Rejection) -> Result<impl Reply, Rejection> {
    let e = e.find::<ErrorResponse>().map_or_else(
        || error_response(StatusCode::INTERNAL_SERVER_ERROR, "Unhandled exception"),
        |e| e.to_owned(),
    );
    let code = StatusCode::from_u16(e.get_code()).expect("Constructed from StatusCode");
    let json = warp::reply::json(&e);
    Ok(warp::reply::with_status(json, code))
}
