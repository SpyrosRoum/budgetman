use askama::Template;

#[derive(Template)]
#[template(path = "500.html")]
pub(crate) struct View500 {
    pub(crate) username: String,
}

#[derive(Template)]
#[template(path = "404.html")]
pub(crate) struct View404 {
    pub(crate) username: String,
}
