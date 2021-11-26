use {
    common::{
        models::user::UserRow,
        with_db,
    },
    sqlx::SqlitePool,
    warp::{Filter, Rejection},
};

pub(crate) fn require_login(
    db: SqlitePool,
) -> impl Filter<Extract = (UserRow,), Error = Rejection> + Clone {
    warp::cookie::optional("access_token")
        .and(with_db(db))
        .and_then(api::check_token)
}
