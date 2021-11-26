use {
    sea_query::{self, Iden},
    serde::{Deserialize, Serialize},
    sqlx,
};

#[derive(Iden)]
pub enum UserTable {
    #[iden = "users"]
    Table,
    Id,
    Username,
    PasswordHash,
    Admin,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UserRow {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub admin: bool,
}

/// Ways to identify a user
pub enum UserIdent {
    Id(String),
    Username(String),
}

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub id: String,
}
