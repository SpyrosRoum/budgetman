use {
    sea_query::{self, Iden},
    serde::{Deserialize, Serialize},
    strum::EnumIter,
};

#[derive(Iden, EnumIter)]
pub(crate) enum AccountTable {
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
/// Can be any account
pub(crate) struct AccountRow {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) available_money: Option<f64>,
    pub(crate) total_money: Option<f64>,
    pub(crate) user_id: String,
    pub(crate) is_adhoc: bool,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
/// Specific for normal accounts
pub(crate) struct NormalAccountRow {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) available_money: f64,
    pub(crate) total_money: f64,
    pub(crate) user_id: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
/// Specific for adhoc accounts
pub(crate) struct AdhocAccountRow {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum AccountType {
    #[serde(alias = "any")]
    #[serde(alias = "*")]
    #[serde(alias = "all")]
    Any,
    #[serde(alias = "adhoc")]
    Adhoc,
    #[serde(alias = "normal")]
    Normal,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct AccountTypeQuery {
    #[serde(alias = "type")]
    pub(crate) account_type: AccountType,
}

impl Default for AccountTypeQuery {
    fn default() -> Self {
        Self {
            account_type: AccountType::Any,
        }
    }
}
