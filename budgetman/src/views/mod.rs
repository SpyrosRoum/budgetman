mod account;

use {
    askama::Template,
    warp::{Filter, Rejection, Reply},
};

pub(crate) fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let index = warp::path::end().and(warp::get()).map(|| Index {
        // Eventually through authentication we will have an actual user and his username
        user_name: "blah".to_owned(),
    });
    let account_pages = account::routes();

    index.or(account_pages)
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    user_name: String,
}
