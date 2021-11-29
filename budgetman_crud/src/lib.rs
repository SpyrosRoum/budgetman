use std::env;

use {
    anyhow::Context,
    common::{
        auth::{create_jwt, validate_password},
        err_resp,
        models::{account::*, user::*},
        requests::LoginRequest,
        responses::ErrorResponse,
    },
    sea_query::{self, bind_params_sqlx_sqlite, Expr, Func, Query, SqliteQueryBuilder, Value},
    sqlx::SqlitePool,
    uuid::Uuid,
    warp::http::StatusCode,
};

pub async fn create_db() -> anyhow::Result<SqlitePool> {
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
pub async fn add_default_user(db: &SqlitePool) -> anyhow::Result<()> {
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
                common::auth::hash_password("admin").unwrap().into(),
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

pub async fn fetch_user_from(
    db: &SqlitePool,
    ident: &UserIdent,
) -> anyhow::Result<Option<UserRow>> {
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

    query
        .fetch_optional(db)
        .await
        .context("Failed to fetch user from db")
}

/// Try to validate the username and password, if successful get a jwt
pub async fn login(req: LoginRequest, db: &SqlitePool) -> Result<String, ErrorResponse> {
    let user = fetch_user_from(db, &UserIdent::Username(req.username.clone()))
        .await
        .map_err(|e| err_resp(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| err_resp(StatusCode::UNAUTHORIZED, "Incorrect username or password"))?;

    if !validate_password(&user.password_hash, &req.password) {
        return Err(err_resp(
            StatusCode::UNAUTHORIZED,
            "Incorrect username or password",
        ));
    }

    Ok(
        create_jwt(&user)
            .map_err(|e| err_resp(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    )
}

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
