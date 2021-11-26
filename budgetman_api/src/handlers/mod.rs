use {
    serde_json::json,
    sqlx::SqlitePool,
    warp::{Rejection, Reply},
};

pub(crate) async fn handle_login(
    req: common::requests::LoginRequest,
    db: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let jwt = crud::login(req, &db).await?;
    Ok(warp::reply::json(&json!({ "access_token": jwt })))
}
