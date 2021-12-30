use {
    askama::Template,
    axum::{
        extract::{Extension, Form},
        http::Uri,
        response::Redirect,
        routing::get,
        Router,
    },
    sqlx::SqlitePool,
    tower_cookies::{Cookie, Cookies},
};

use crate::{html_template::HtmlTemplate, requests::LoginRequest, Error};

pub(crate) fn routes() -> Router {
    Router::new().route(
        "/login",
        get(|| async { HtmlTemplate(LogIn) }).post(handle_login),
    )
}

// Post /login
pub(crate) async fn handle_login(
    Extension(db): Extension<SqlitePool>,
    Form(req): Form<LoginRequest>,
    cookies: Cookies,
) -> Result<Redirect, Error> {
    let jwt = crate::crud::login(req, &db)
        .await
        .map_err(Error::HtmlError)?;

    let mut cookie = Cookie::new("access_token", jwt);
    cookie.set_http_only(true);
    cookies.add(cookie);

    Ok(Redirect::to(Uri::from_static("/")))
}

#[derive(Template)]
#[template(path = "account/login.html")]
pub(crate) struct LogIn;
