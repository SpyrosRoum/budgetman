use {
    common::models::{
        account::{AccountType, AccountTypeQuery},
        user::UserRow,
    },
    serde_json::json,
    sqlx::SqlitePool,
    warp::{Rejection, Reply},
};

/// Post /api/v1/login
pub(crate) async fn handle_login(
    req: common::requests::LoginRequest,
    db: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let jwt = crud::login(req, &db).await?;
    Ok(warp::reply::json(&json!({ "access_token": jwt })))
}

/// Get /api/v1/accounts
pub(crate) async fn get_accounts(
    db: SqlitePool,
    user: UserRow,
    type_q: AccountTypeQuery,
) -> Result<impl Reply, Rejection> {
    let json = match type_q.account_type {
        AccountType::Any => {
            let accounts = crud::fetch_accounts(&db, &user.id).await?;
            json!(accounts)
        }
        AccountType::Adhoc => {
            let accounts = crud::fetch_adhoc_accounts(&db, &user.id).await?;
            json!(accounts)
        }
        AccountType::Normal => {
            let accounts = crud::fetch_normal_accounts(&db, &user.id).await?;
            json!(accounts)
        }
    };

    Ok(warp::reply::json(&json))
}
