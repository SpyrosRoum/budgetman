use {
    common::{
        err_resp, models::account::*, requests::AccountCreateRequest, responses::ErrorResponse,
    },
    sea_query::{bind_params_sqlx_sqlite, Expr, Query, SqliteQueryBuilder, Value},
    sqlx::SqlitePool,
    warp::http::StatusCode,
};

/// Get accounts related to the given `user_id`
pub async fn fetch_accounts(
    db: &SqlitePool,
    user_id: &str,
) -> Result<Vec<AccountRow>, ErrorResponse> {
    let (sql, values) = Query::select()
        .columns([
            AccountTable::Id,
            AccountTable::Name,
            AccountTable::Description,
            AccountTable::AvailableMoney,
            AccountTable::TotalMoney,
            AccountTable::UserId,
            AccountTable::IsAdhoc,
        ])
        .from(AccountTable::Table)
        .and_where(Expr::col(AccountTable::UserId).eq(user_id.to_owned()))
        .build(SqliteQueryBuilder);
    let query = bind_params_sqlx_sqlite!(sqlx::query_as(&sql), values);

    query.fetch_all(db).await.map_err(|_e| {
        err_resp(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch accounts for user {}", user_id),
        )
    })
}

pub async fn fetch_adhoc_accounts(
    db: &SqlitePool,
    user_id: &str,
) -> Result<Vec<AdhocAccountRow>, ErrorResponse> {
    let (sql, values) = Query::select()
        .columns([AccountTable::Id, AccountTable::Name, AccountTable::UserId])
        .from(AccountTable::Table)
        .and_where(Expr::col(AccountTable::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(AccountTable::IsAdhoc).eq(true))
        .build(SqliteQueryBuilder);
    let query = bind_params_sqlx_sqlite!(sqlx::query_as(&sql), values);

    query.fetch_all(db).await.map_err(|_| {
        err_resp(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch adhoc accounts for user {}", user_id),
        )
    })
}

pub async fn fetch_normal_accounts(
    db: &SqlitePool,
    user_id: &str,
) -> Result<Vec<NormalAccountRow>, ErrorResponse> {
    let (sql, values) = Query::select()
        .columns([
            AccountTable::Id,
            AccountTable::Name,
            AccountTable::Description,
            AccountTable::AvailableMoney,
            AccountTable::TotalMoney,
            AccountTable::UserId,
        ])
        .from(AccountTable::Table)
        .and_where(Expr::col(AccountTable::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(AccountTable::IsAdhoc).eq(false))
        .build(SqliteQueryBuilder);
    let query = bind_params_sqlx_sqlite!(sqlx::query_as(&sql), values);

    query.fetch_all(db).await.map_err(|_| {
        err_resp(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch normal accounts for user {}", user_id),
        )
    })
}

/// Create the given account.
/// If it's a normal account but starting_money is None then set it to 0
/// Returns the created account's id if successful
pub async fn create_account(
    db: &SqlitePool,
    user_id: &str,
    acc: AccountCreateRequest,
) -> Result<i64, ErrorResponse> {
    // If the account is adhoc we want to set money to None no matter what,
    // same with description
    let (money, description) = match acc.is_adhoc {
        true => (None, None),
        false => (Some(acc.starting_money.unwrap_or(0.0)), acc.description),
    };

    let (sql, values) = Query::insert()
        .into_table(AccountTable::Table)
        .columns([
            AccountTable::Name,
            AccountTable::Description,
            AccountTable::AvailableMoney,
            AccountTable::TotalMoney,
            AccountTable::UserId,
            AccountTable::IsAdhoc,
        ])
        .values_panic([
            acc.name.into(),
            description.into(),
            money.into(),
            money.into(),
            user_id.into(),
            acc.is_adhoc.into(),
        ])
        .build(SqliteQueryBuilder);

    let query = bind_params_sqlx_sqlite!(sqlx::query(&sql), values);
    let r = query.execute(db).await.map_err(|e| {
        if let Some(e) = e.as_database_error() {
            let code = if e.message() == "UNIQUE constraint failed: accounts.name" {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            err_resp(code, format!("Failed to insert account: {}", e.message()))
        } else {
            err_resp(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert account: {}", e.to_string()),
            )
        }
    })?;

    Ok(r.last_insert_rowid())
}
