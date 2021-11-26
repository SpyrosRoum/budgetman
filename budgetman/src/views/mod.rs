mod account;
mod views_404_500;

use {
    askama::Template,
    common::{models::user::UserRow, responses::ErrorResponse},
    sqlx::SqlitePool,
    warp::{
        http::{StatusCode, Uri},
        Filter, Rejection, Reply,
    },
};

use crate::utils::require_login;

pub(crate) fn routes(
    db: &SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let index = warp::path::end()
        .and(warp::get())
        .and(require_login(db.clone()))
        .map(|u: UserRow| Index {
            username: u.username,
        });

    let account_pages = account::routes(db);

    index
        .or(account_pages)
        .or(warp::path!("500").map(|| views_404_500::View500 {
            username: "Really not sure, could *try* to authenticate and get a username but meh"
                .into(),
        }))
        .or(warp::path!("404").map(|| views_404_500::View404 {
            username: "Really not sure, could *try* to authenticate and get a username but meh"
                .into(),
        }))
        .recover(handle_rejection)
}

/// Only three things can fail:
/// 1. Authentication
/// 2. Internal error (for example something went wrong with the db)
/// 3 . 404
async fn handle_rejection(r: Rejection) -> Result<impl Reply, Rejection> {
    let uri = if let Some(e) = r.find::<ErrorResponse>() {
        if e.get_code() == StatusCode::INTERNAL_SERVER_ERROR.as_u16() {
            Uri::from_static("/500")
        } else {
            Uri::from_static("/login")
        }
    } else if r.is_not_found() {
        Uri::from_static("/404")
    } else {
        Uri::from_static("/500")
    };

    Ok(warp::redirect(uri))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    username: String,
}
