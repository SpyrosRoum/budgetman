use warp::{http::Uri, Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // Might want to play with GraphQL later or simply do breaking changes to the api, so we use `v1` path
    let base = warp::path!("api" / "v1" / ..);

    let login = base
        .and(warp::path!("login"))
        .and(warp::post())
        .and(warp::body::json())
        .map(|map: common::LoginRequest| {
            println!("Log In: {:?}", map);
            warp::redirect(Uri::from_static("/"))
        });

    login
}
