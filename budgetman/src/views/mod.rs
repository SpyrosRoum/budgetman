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

pub(crate) fn routes(
    db: &SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let index = warp::path::end()
        .and(warp::get())
        .and(api::utils::require_login(db.clone()))
        .map(|u: UserRow| Index {
            username: u.username,
        });

    let account_pages = account::routes(db);

    index
        .or(account_pages)
        // ToDo: Try to authenticate and if successful use the user's name
        .or(warp::path!("500").map(|| views_404_500::View500::new(None)))
        .or(warp::path!("404").map(|| views_404_500::View404::new(None)))
        .recover(handle_rejection)
}

/// Handle rejections that can occur when creating views:
/// 1. Authentication
/// 2. Internal error (for example something went wrong with the db)
async fn handle_rejection(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(e) = r.find::<ErrorResponse>() {
        let uri = if e.get_code() == StatusCode::INTERNAL_SERVER_ERROR.as_u16() {
            Uri::from_static("/500")
        } else {
            Uri::from_static("/login")
        };

        Ok(warp::redirect(uri))
    } else {
        Err(r)
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    username: String,
}
