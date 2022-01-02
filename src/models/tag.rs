use {
    sea_query::{self, Iden},
    serde::{Deserialize, Serialize},
    sqlx::types::BigDecimal,
    strum::EnumIter,
    uuid::Uuid,
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
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) limit: Option<BigDecimal>,
    pub(crate) balance: BigDecimal,
    pub(crate) user_id: Uuid,
}
