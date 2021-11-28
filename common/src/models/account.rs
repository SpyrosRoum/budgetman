use {
    sea_query::{self, Iden},
    serde::{Deserialize, Serialize},
    sqlx,
};

#[derive(Iden)]
pub enum AccountTable {
    #[iden = "accounts"]
    Table,
    Id,
    Name,
    Description,
    AvailableMoney,
    TotalMoney,
    UserId,
    IsAdhoc,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct AccountRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub available_money: Option<f64>,
    pub total_money: Option<f64>,
    pub user_id: String,
    pub is_adhoc: bool,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct NormalAccountRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub available_money: f64,
    pub total_money: f64,
    pub user_id: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct AdhocAccountRow {
    pub id: i64,
    pub name: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AccountType {
    #[serde(alias = "any")]
    #[serde(alias = "*")]
    #[serde(alias = "all")]
    Any,
    #[serde(alias = "adhoc")]
    Adhoc,
    #[serde(alias = "normal")]
    Normal,
}

#[derive(Deserialize, Serialize)]
pub struct AccountTypeQuery {
    #[serde(alias = "type")]
    pub account_type: AccountType,
}

impl AccountTypeQuery {
    pub fn new(account_type: AccountType) -> Self {
        Self { account_type }
    }
}
