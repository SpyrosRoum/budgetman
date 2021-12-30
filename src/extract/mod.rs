//! Various extractors that return a Json error instead of a plain string

mod json;
mod query;

pub(crate) use {json::Json, query::Query};
