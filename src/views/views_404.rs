use askama::Template;

#[derive(Template)]
#[template(path = "404.html")]
pub(crate) struct View404 {
    pub(crate) username: String,
}

impl View404 {
    pub(crate) fn new(username: Option<&str>) -> Self {
        let username = username.map_or_else(String::new, String::from);
        Self { username }
    }
}
