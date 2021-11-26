use {
    askama::Template,
    common::{requests::LoginRequest, with_db},
    sqlx::SqlitePool,
    warp::{
        http::{header, Response, StatusCode},
        Filter, Rejection, Reply,
    },
};

pub(crate) fn routes(
    db: &SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let login = warp::path!("login").and(warp::get()).map(|| LogIn);
    let login_request = warp::path!("login")
        .and(warp::post())
        .and(warp::body::form())
        .and(with_db(db.clone()))
        .and_then(handle_login);

    login.or(login_request)
}

async fn handle_login(req: LoginRequest, db: SqlitePool) -> Result<impl Reply, Rejection> {
    let jwt = crud::login(req, &db).await?;
    Ok(Response::builder()
        .header(header::LOCATION, "/")
        .status(StatusCode::MOVED_PERMANENTLY)
        .header(
            header::SET_COOKIE,
            format!("access_token={}; HttpOnly", jwt),
        )
        .body("")
        .unwrap())
}

#[derive(Template)]
#[template(path = "account/login.html")]
struct LogIn;
