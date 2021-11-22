mod entities;

use std::env;

use {sea_orm::DatabaseConnection, sqlx::SqlitePool, uuid::Uuid};

pub async fn create_db() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

/// Add the default admin user to the database if there are no users
pub async fn add_default_user(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    use entities::user;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};

    if user::Entity::find()
        .filter(user::Column::Admin.eq(true))
        .count(db)
        .await
        .unwrap()
        == 0
    {
        let user = user::ActiveModel {
            id: Set(Uuid::new_v4().to_hyphenated().to_string()),
            username: Set(String::from("admin")),
            password_hash: Set(common::hash_password("admin").unwrap()),
            admin: Set(true),
        };

        user::ActiveModel::insert(user, db).await?;
    }

    Ok(())
}
