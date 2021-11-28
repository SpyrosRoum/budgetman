mod views;

use std::{convert::Infallible, env};

use {
    anyhow::Context,
    dotenv::dotenv,
    warp::{http::Uri, Filter, Rejection, Reply},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db = crud::create_db().await?;
    if let Err(e) = crud::add_default_user(&db).await {
        eprint!("Failed to add default admin user: {:#}", e);
    }

    let secret = env::var("SECRET").context("Expected `SECRET` env variable")?;
    common::set_secret(&secret);

    let api = api::routes(&db);
    let views = views::routes(&db);
    let admin_lte = warp::path("static").and(warp::fs::dir("static"));

    let routes = admin_lte.or(views).or(api).recover(handle_rejection);

    let server = warp::serve(routes).run(([0, 0, 0, 0], 8000));
    println!("Listening at http://0.0.0.0:8000");
    server.await;

    Ok(())
}

/// Catch any unhandled routes (404) or internal errors (505)
/// All internal errors must have been caught earlier but we check it here as well just in case
async fn handle_rejection(r: Rejection) -> Result<impl Reply, Infallible> {
    let uri = if r.is_not_found() {
        Uri::from_static("/404")
    } else {
        Uri::from_static("/500")
    };

    Ok(warp::redirect(uri))
}
