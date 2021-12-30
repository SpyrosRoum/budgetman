use {
    axum::{extract::Extension, http::StatusCode},
    serde_json::{json, Value},
    sqlx::SqlitePool,
};

use crate::{
    crud,
    extract::{Json, Query},
    models::{account::*, user::UserRow},
    requests::AccountCreateRequest,
    Error,
};

/// Post /api/v1/login
pub(crate) async fn handle_login(
    Json(req): Json<crate::requests::LoginRequest>,
    Extension(db): Extension<SqlitePool>,
) -> Result<Json<Value>, Error> {
    let jwt = crud::login(req, &db).await.map_err(Error::ApiError)?;
    Ok(Json(json!({ "access_token": jwt })))
}

/// Get /api/v1/accounts
pub(crate) async fn get_accounts(
    Extension(db): Extension<SqlitePool>,
    user: UserRow,
    Query(type_q): Query<AccountTypeQuery>,
) -> Result<Json<Value>, Error> {
    tracing::info!("{:?}", type_q);
    let accounts = match type_q.account_type {
        AccountType::Any => {
            let accounts = crud::accounts::fetch_accounts(&db, &user.id)
                .await
                .map_err(Error::ApiError)?;
            json!(accounts)
        }
        AccountType::Adhoc => {
            let accounts = crud::accounts::fetch_adhoc_accounts(&db, &user.id)
                .await
                .map_err(Error::ApiError)?;
            json!(accounts)
        }
        AccountType::Normal => {
            let accounts = crud::accounts::fetch_normal_accounts(&db, &user.id)
                .await
                .map_err(Error::ApiError)?;
            json!(accounts)
        }
    };

    Ok(Json(accounts))
}

/// Post /api/v1/accounts
pub(crate) async fn create_account(
    Extension(db): Extension<SqlitePool>,
    user: UserRow,
    Json(account): Json<AccountCreateRequest>,
) -> Result<Json<Value>, Error> {
    if !account.is_adhoc && account.starting_money.is_none() {
        let err = (
            StatusCode::BAD_REQUEST,
            "Normal account needs initial money",
        )
            .into();
        return Err(Error::ApiError(err));
    }
    let id = crud::accounts::create_account(&db, &user.id, account)
        .await
        .map_err(Error::ApiError)?;
    Ok(Json(json!({ "id": id })))
}
