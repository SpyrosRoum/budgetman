use std::borrow::Cow;

use {
    sea_query::{bind_params_sqlx_postgres, Expr, PostgresQueryBuilder, Query, Value},
    sqlx::PgPool,
    strum::IntoEnumIterator,
    uuid::Uuid,
};

use crate::{models::tag::*, requests::TagCreate, utils, CommonError};

pub(crate) async fn fetch_tags(db: &PgPool, user_id: Uuid) -> Result<Vec<TagRow>, CommonError> {
    let (sql, values) = Query::select()
        .columns(TagTable::iter().skip(1))
        .from(TagTable::Table)
        .and_where(Expr::col(TagTable::UserId).eq(user_id))
        .build(PostgresQueryBuilder);
    let query = bind_params_sqlx_postgres!(sqlx::query_as(&sql), values);
    Ok(query.fetch_all(db).await?)
}

pub(crate) async fn fetch_tag(db: &PgPool, user_id: Uuid, id: i32) -> Result<TagRow, CommonError> {
    let (sql, values) = Query::select()
        .columns(TagTable::iter().skip(1))
        .from(TagTable::Table)
        .and_where(Expr::col(TagTable::UserId).eq(user_id))
        .and_where(Expr::col(TagTable::Id).eq(id))
        .build(PostgresQueryBuilder);
    let query = bind_params_sqlx_postgres!(sqlx::query_as(&sql), values);

    query.fetch_optional(db).await?.ok_or(CommonError::NotFound)
}

pub(crate) async fn create_tag(
    db: &PgPool,
    user_id: Uuid,
    tag: TagCreate,
) -> Result<i32, CommonError> {
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
            tag.starting_money.unwrap_or_default().into(),
            user_id.into(),
        ])
        .returning_col(TagTable::Id)
        .build(PostgresQueryBuilder);
    tracing::trace!("SQL: {}", &sql);
    tracing::trace!("Values: {:?}", &values);
    let query = bind_params_sqlx_postgres!(sqlx::query_scalar(&sql), values);
    let r = query.fetch_one(db).await.map_err(|e| {
        let msg = if utils::err_is_failed_constraint(&e) {
            Some(Cow::Borrowed("There already is a tag with that name"))
        } else {
            None
        };

        CommonError::Db { msg, source: e }
    })?;

    Ok(r)
}
