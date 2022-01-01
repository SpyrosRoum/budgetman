use {
    axum::{
        extract::{Extension, Path},
        http::StatusCode,
    },
    serde_json::{json, Value},
    sqlx::SqlitePool,
};

use crate::{
    crud,
    extract::{Json, Query},
    models::{account::*, tag::TagRow, user::UserRow},
    requests::*,
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

/// Get /api/v1/accounts/:id
pub(crate) async fn get_specific_account(
    Extension(db): Extension<SqlitePool>,
    user: UserRow,
    Path(id): Path<i64>,
) -> Result<Json<AccountRow>, Error> {
    let account = crud::accounts::fetch_account(&db, &user.id, id)
        .await
        .map_err(Error::ApiError)?;
    Ok(Json(account))
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

/// Get /api/v1/tags
pub(crate) async fn get_tags(
    Extension(db): Extension<SqlitePool>,
    user: UserRow,
) -> Result<Json<Vec<TagRow>>, Error> {
    let tags = crud::tags::fetch_tags(&db, &user.id)
        .await
        .map_err(Error::ApiError)?;
    Ok(Json(tags))
}

/// Get /api/v1/tags
pub(crate) async fn get_specific_tag(
    Extension(db): Extension<SqlitePool>,
    user: UserRow,
    Path(id): Path<i64>,
) -> Result<Json<TagRow>, Error> {
    let tags = crud::tags::fetch_tag(&db, &user.id, id)
        .await
        .map_err(Error::ApiError)?;
    Ok(Json(tags))
}

/// Get /api/v1/tags
pub(crate) async fn create_tag(
    Extension(db): Extension<SqlitePool>,
    user: UserRow,
    Json(to_create): Json<TagCreate>,
) -> Result<Json<Value>, Error> {
    let id = crud::tags::create_tag(&db, user.id, to_create)
        .await
        .map_err(Error::ApiError)?;
    Ok(Json(json!({ "id": id })))
}
