use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Text")]
    pub id: String,
    #[sea_orm(unique)]
    pub username: String,
    pub password_hash: String,
    pub admin: bool,
}

#[derive(EnumIter, Debug, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
