use std::borrow::Cow;

use {
    sea_query::{bind_params_sqlx_sqlite, Expr, Query, SqliteQueryBuilder, Value},
    sqlx::SqlitePool,
    strum::IntoEnumIterator,
};

use crate::{models::tag::*, requests::TagCreate, utils, CommonError};

pub(crate) async fn fetch_tags(db: &SqlitePool, user_id: &str) -> Result<Vec<TagRow>, CommonError> {
    let (sql, values) = Query::select()
        .columns(TagTable::iter().skip(1))
        .from(TagTable::Table)
        .and_where(Expr::col(TagTable::UserId).eq(user_id))
        .build(SqliteQueryBuilder);
    let query = bind_params_sqlx_sqlite!(sqlx::query_as(&sql), values);
    Ok(query.fetch_all(db).await?)
}

pub(crate) async fn fetch_tag(
    db: &SqlitePool,
    user_id: &str,
    id: i64,
) -> Result<TagRow, CommonError> {
    let (sql, values) = Query::select()
        .columns(TagTable::iter().skip(1))
        .from(TagTable::Table)
        .and_where(Expr::col(TagTable::UserId).eq(user_id))
        .and_where(Expr::col(TagTable::Id).eq(id))
        .build(SqliteQueryBuilder);
    let query = bind_params_sqlx_sqlite!(sqlx::query_as(&sql), values);

    query.fetch_optional(db).await?.ok_or(CommonError::NotFound)
}

pub(crate) async fn create_tag(
    db: &SqlitePool,
    user_id: String,
    tag: TagCreate,
) -> Result<i64, CommonError> {
    let (sql, values) = Query::insert()
        .into_table(TagTable::Table)
        .columns([
            TagTable::Name,
            TagTable::Description,
            TagTable::Limit,
            TagTable::Balance,
            TagTable::UserId,
        ])
        .values_panic([
            tag.name.into(),
            tag.description.into(),
            tag.limit.into(),
            tag.starting_money.unwrap_or(0.0).into(),
            user_id.into(),
        ])
        .build(SqliteQueryBuilder);
    tracing::trace!("SQL: {}", &sql);
    tracing::trace!("Values: {:?}", &values);
    let query = bind_params_sqlx_sqlite!(sqlx::query(&sql), values);
    let r = query.execute(db).await.map_err(|e| {
        let msg = if utils::err_is_failed_constraint(&e) {
            Some(Cow::Borrowed("There already is a tag with that name"))
        } else {
            None
        };

        CommonError::Db { msg, source: e }
    })?;

    Ok(r.last_insert_rowid())
}
