use {
    common::{
        err_resp,
        models::{account::*, user::UserRow},
        requests::AccountCreateRequest,
    },
    serde_json::json,
    sqlx::SqlitePool,
    warp::{http::StatusCode, Rejection, Reply},
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
            let accounts = crud::accounts::fetch_accounts(&db, &user.id).await?;
            json!(accounts)
        }
        AccountType::Adhoc => {
            let accounts = crud::accounts::fetch_adhoc_accounts(&db, &user.id).await?;
            json!(accounts)
        }
        AccountType::Normal => {
            let accounts = crud::accounts::fetch_normal_accounts(&db, &user.id).await?;
            json!(accounts)
        }
    };

    Ok(warp::reply::json(&json))
}

/// Post /api/v1/accounts
pub(crate) async fn create_adhoc_account(
    db: SqlitePool,
    user: UserRow,
    account: AccountCreateRequest,
) -> Result<impl Reply, Rejection> {
    if !account.is_adhoc && account.starting_money.is_none() {
        return Err(err_resp(
            StatusCode::BAD_REQUEST,
            "Normal account needs initial money",
        )
        .into());
    }
    let id = crud::accounts::create_account(&db, &user.id, account).await?;
    Ok(warp::reply::json(&json!({ "id": id })))
}
