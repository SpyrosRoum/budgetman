pub(crate) mod accounts;

use std::env;

use {
    anyhow::Context,
    sea_query::{self, bind_params_sqlx_sqlite, Expr, Func, Query, SqliteQueryBuilder, Value},
    sqlx::SqlitePool,
    uuid::Uuid,
};

use crate::{
    models::user::*,
    requests::LoginRequest,
    utils::auth::{create_jwt, validate_password},
    CommonError,
};

pub(crate) async fn create_db() -> anyhow::Result<SqlitePool> {
    let db_url = env::var("DATABASE_URL").context("Expected `DATABASE_URL` env variable")?;
    let pool = SqlitePool::connect(&db_url)
        .await
        .context("Failed to connect to db")?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Failed to run migrations")?;
    Ok(pool)
}

/// Add the default admin user to the database if there are no users
pub(crate) async fn add_default_user(db: &SqlitePool) -> anyhow::Result<()> {
    let (sql, values) = Query::select()
        .from(UserTable::Table)
        .and_where(Expr::col(UserTable::Admin).eq(true))
        .expr(Func::count(Expr::col(UserTable::Id)))
        .build(SqliteQueryBuilder);
    let query = bind_params_sqlx_sqlite!(sqlx::query_scalar(&sql), values);
    let count: u32 = query.fetch_one(db).await.context("DB error")?;

    if count == 0 {
        let (sql, values) = Query::insert()
            .into_table(UserTable::Table)
            .columns([
                UserTable::Id,
                UserTable::Username,
                UserTable::PasswordHash,
                UserTable::Admin,
            ])
            .values_panic([
                Uuid::new_v4().to_string().into(),
                "admin".into(),
                crate::utils::auth::hash_password("admin").unwrap().into(),
                true.into(),
            ])
            .build(SqliteQueryBuilder);
        let query = bind_params_sqlx_sqlite!(sqlx::query(&sql), values);
        query
            .execute(db)
            .await
            .context("Failed to insert default admin user")?;
    }

    Ok(())
}

pub(crate) async fn fetch_user_from(
    db: &SqlitePool,
    ident: &UserIdent,
) -> Result<Option<UserRow>, CommonError> {
    let expr = match ident {
        UserIdent::Id(id) => Expr::col(UserTable::Id).eq(id.as_str()),
        UserIdent::Username(username) => Expr::col(UserTable::Username).eq(username.as_str()),
    };

    let (sql, values) = Query::select()
        .columns([
            UserTable::Id,
            UserTable::Username,
            UserTable::PasswordHash,
            UserTable::Admin,
        ])
        .from(UserTable::Table)
        .and_where(expr)
        .build(SqliteQueryBuilder);
    let query = bind_params_sqlx_sqlite!(sqlx::query_as(&sql), values);

    Ok(query
        .fetch_optional(db)
        .await
        .map_err(|e| CommonError::Db {
            msg: String::from("Failed to fetch user from db"),
            source: e,
        })?)
}

/// Try to validate the username and password, if successful get a jwt
pub(crate) async fn login(req: LoginRequest, db: &SqlitePool) -> Result<String, CommonError> {
    let user = fetch_user_from(db, &UserIdent::Username(req.username.clone()))
        .await?
        .ok_or(CommonError::WrongCredentials)?;

    if !validate_password(&user.password_hash, &req.password) {
        return Err(CommonError::WrongCredentials);
    }

    Ok(create_jwt(&user)?)
}
