use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub starting_money: Option<f64>,
    pub is_adhoc: bool,
}
