mod account;
pub(crate) mod views_404;
pub(crate) mod views_500;

use {
    askama::Template,
    axum::{routing::get, Router},
};

use crate::{html_template::HtmlTemplate, models::user::UserRow};

pub(crate) fn routes() -> Router {
    let views = Router::new()
        .route(
            "/",
            get(|user: UserRow| async {
                HtmlTemplate(Index {
                    username: user.username,
                })
            }),
        )
        .route(
            "/500",
            get(|user: Option<UserRow>| async {
                HtmlTemplate(views_500::View500::new(user.map(|u| u.username).as_deref()))
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
