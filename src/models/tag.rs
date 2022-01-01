use {
    sea_query::{self, Iden},
    serde::{Deserialize, Serialize},
    strum::EnumIter,
};

#[derive(Iden, EnumIter)]
pub(crate) enum TagTable {
    #[iden = "tags"]
    Table,
    Id,
    Name,
    Description,
    Limit,
    Balance,
    UserId,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub(crate) struct TagRow {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) limit: Option<f64>,
    pub(crate) balance: f64,
    pub(crate) user_id: String,
}
