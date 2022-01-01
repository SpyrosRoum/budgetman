use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AccountCreateRequest {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) starting_money: Option<f64>,
    pub(crate) is_adhoc: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TagCreate {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) limit: Option<f64>,
    pub(crate) starting_money: Option<f64>,
}
