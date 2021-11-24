mod views;

use {dotenv::dotenv, warp::Filter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db = crud::create_db().await?;
    if let Err(e) = crud::add_default_user(&db).await {
        eprint!("Failed to add default admin user: {:#}", e);
    }

    let api = api::routes(&db);
    let views = views::routes();
    let admin_lte = warp::path("static").and(warp::fs::dir("static"));

    let routes = admin_lte.or(views).or(api);

    let server = warp::serve(routes).run(([0, 0, 0, 0], 8000));
    println!("Listening at http://0.0.0.0:8000");
    server.await;

    Ok(())
}
