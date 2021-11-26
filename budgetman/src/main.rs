mod utils;
mod views;

use std::env;

use {anyhow::Context, dotenv::dotenv, warp::Filter};

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

    let routes = admin_lte.or(views).or(api);

    let server = warp::serve(routes).run(([0, 0, 0, 0], 8000));
    println!("Listening at http://0.0.0.0:8000");
    server.await;

    Ok(())
}
