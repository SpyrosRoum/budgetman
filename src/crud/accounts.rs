use std::borrow::Cow;

use {
    sea_query::{bind_params_sqlx_postgres, Expr, PostgresQueryBuilder, Query, Value},
    sqlx::PgPool,
    strum::IntoEnumIterator,
    uuid::Uuid,
};

use crate::{models::account::*, requests::AccountCreateRequest, utils, CommonError};

/// Get accounts related to the given `user_id`
pub(crate) async fn fetch_accounts(
    db: &PgPool,
    user_id: &Uuid,
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
        .build(PostgresQueryBuilder);
    let query = bind_params_sqlx_postgres!(sqlx::query_as(&sql), values);

    Ok(query.fetch_all(db).await.map_err(|e| CommonError::Db {
        msg: Some("Failed to fetch accounts from db".into()),
        source: e,
    })?)
}

pub(crate) async fn fetch_adhoc_accounts(
    db: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<AdhocAccountRow>, CommonError> {
    let (sql, values) = Query::select()
        .columns([AccountTable::Id, AccountTable::Name, AccountTable::UserId])
        .from(AccountTable::Table)
        .and_where(Expr::col(AccountTable::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(AccountTable::IsAdhoc).eq(true))
        .build(PostgresQueryBuilder);
    let query = bind_params_sqlx_postgres!(sqlx::query_as(&sql), values);

    Ok(query.fetch_all(db).await.map_err(|e| CommonError::Db {
        msg: Some("Failed to fetch adhoc accounts from db".into()),
        source: e,
    })?)
}

pub(crate) async fn fetch_account(
    db: &PgPool,
    user_id: &Uuid,
    account_id: i32,
) -> Result<AccountRow, CommonError> {
    let (sql, values) = Query::select()
        .columns(AccountTable::iter().skip(1))
        .from(AccountTable::Table)
        .and_where(Expr::col(AccountTable::Id).eq(account_id))
        .and_where(Expr::col(AccountTable::UserId).eq(user_id.to_owned()))
        .build(PostgresQueryBuilder);
    let query = bind_params_sqlx_postgres!(sqlx::query_as(&sql), values);
    let account: Option<AccountRow> = query.fetch_optional(db).await?;
    account.ok_or(CommonError::NotFound)
}

pub(crate) async fn fetch_normal_accounts(
    db: &PgPool,
    user_id: &Uuid,
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
        .build(PostgresQueryBuilder);
    let query = bind_params_sqlx_postgres!(sqlx::query_as(&sql), values);

    Ok(query.fetch_all(db).await.map_err(|e| CommonError::Db {
        msg: Some("Failed to fetch normal accounts from db".into()),
        source: e,
    })?)
}

/// Create the given account.
/// If it's a normal account but starting_money is None then set it to 0.
///
/// Returns the created account's id if successful.
pub(crate) async fn create_account(
    db: &PgPool,
    user_id: Uuid,
    acc: AccountCreateRequest,
) -> Result<i32, CommonError> {
    // If the account is adhoc we want to set money to None no matter what,
    // same with description
    let (money, description) = match acc.is_adhoc {
        true => (None, None),
        false => (
            Some(acc.starting_money.unwrap_or_default()),
            acc.description,
        ),
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
            money.clone().into(),
            money.into(),
            user_id.into(),
            acc.is_adhoc.into(),
        ])
        .returning_col(AccountTable::Id)
        .build(PostgresQueryBuilder);

    let query = bind_params_sqlx_postgres!(sqlx::query_scalar(&sql), values);
    let r = query.fetch_one(db).await.map_err(|e| {
        let msg = if utils::err_is_failed_constraint(&e) {
            Some(Cow::Borrowed("There already is an account with that name"))
        } else {
            None
        };

        CommonError::Db { msg, source: e }
    })?;

    Ok(r)
}
