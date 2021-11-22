mod views;

use {dotenv::dotenv, sea_orm::SqlxSqliteConnector, warp::Filter};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool = crud::create_db().await.unwrap();
    let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(db_pool);
    crud::add_default_user(&db)
        .await
        .expect("Failed to add default admin user");

    let api = api::routes();
    let views = views::routes();
    let admin_lte = warp::path("static").and(warp::fs::dir("static"));

    let routes = admin_lte.or(views).or(api);

    let server = warp::serve(routes).run(([0, 0, 0, 0], 8000));
    println!("Listening at http://0.0.0.0:8000");
    server.await;
}
