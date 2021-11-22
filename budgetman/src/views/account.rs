use {
    askama::Template,
    warp::{http::Uri, Filter, Rejection, Reply},
};

pub(crate) fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let login = warp::path!("login").and(warp::get()).map(|| LogIn);
    let login_request = warp::path!("login")
        .and(warp::post())
        .and(warp::body::form())
        .map(|req: common::LoginRequest| {
            dbg!(&req);
            warp::redirect(Uri::from_static("/"))
        });

    login.or(login_request)
}

#[derive(Template)]
#[template(path = "account/login.html")]
struct LogIn;
