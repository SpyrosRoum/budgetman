mod account;
pub(crate) mod views_404;

use {
    askama::Template,
    axum::{routing::get, Router},
};

use crate::{html_template::HtmlTemplate, models::user::UserRow};

pub(crate) fn routes() -> Router {
    let views = Router::new().route(
        "/",
        get(|user: UserRow| async {
            HtmlTemplate(Index {
                username: user.username,
            })
        }),
    );

    let account_pages = account::routes();

    views.merge(account_pages)
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    username: String,
}
