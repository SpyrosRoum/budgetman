use {
    sea_query::{bind_params_sqlx_sqlite, Expr, Query, SqliteQueryBuilder, Value},
    sqlx::SqlitePool,
};

use crate::{models::account::*, requests::AccountCreateRequest, CommonError};

/// Get accounts related to the given `user_id`
pub(crate) async fn fetch_accounts(
    db: &SqlitePool,
    user_id: &str,
) -> Result<Vec<AccountRow>, CommonError> {
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

    Ok(query.fetch_all(db).await.map_err(|e| CommonError::Db {
        msg: "Failed to fetch accounts from db".to_string(),
        source: e,
    })?)
}

pub(crate) async fn fetch_adhoc_accounts(
    db: &SqlitePool,
    user_id: &str,
) -> Result<Vec<AdhocAccountRow>, CommonError> {
    let (sql, values) = Query::select()
        .columns([AccountTable::Id, AccountTable::Name, AccountTable::UserId])
        .from(AccountTable::Table)
        .and_where(Expr::col(AccountTable::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(AccountTable::IsAdhoc).eq(true))
        .build(SqliteQueryBuilder);
    let query = bind_params_sqlx_sqlite!(sqlx::query_as(&sql), values);

    Ok(query.fetch_all(db).await.map_err(|e| CommonError::Db {
        msg: "Failed to fetch adhoc accounts from db".to_string(),
        source: e,
    })?)
}

pub(crate) async fn fetch_normal_accounts(
    db: &SqlitePool,
    user_id: &str,
) -> Result<Vec<NormalAccountRow>, CommonError> {
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

    Ok(query.fetch_all(db).await.map_err(|e| CommonError::Db {
        msg: "Failed to fetch normal accounts from db".to_string(),
        source: e,
    })?)
}

/// Create the given account.
/// If it's a normal account but starting_money is None then set it to 0.
///
/// Returns the created account's id if successful.
pub(crate) async fn create_account(
    db: &SqlitePool,
    user_id: &str,
    acc: AccountCreateRequest,
) -> Result<i64, CommonError> {
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
        let mut msg = String::from("Failed to insert account");
        if let Some(e) = e.as_database_error() {
            if e.message().starts_with("UNIQUE constraint failed") {
                msg.clear();
                msg.push_str("There already is an account with that name");
            }
        };

        CommonError::Db { msg, source: e }
    })?;

    Ok(r.last_insert_rowid())
}
