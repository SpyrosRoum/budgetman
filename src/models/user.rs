use {
    axum::{
        async_trait,
        extract::{Extension, FromRequest, OriginalUri, RequestParts, TypedHeader},
    },
    headers::{authorization::Bearer, Authorization},
    sea_query::{self, Iden},
    serde::{Deserialize, Serialize},
    sqlx::SqlitePool,
    tower_cookies::Cookies,
};

use crate::{CommonError, Error};

#[derive(Iden)]
pub(crate) enum UserTable {
    #[iden = "users"]
    Table,
    Id,
    Username,
    PasswordHash,
    Admin,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub(crate) struct UserRow {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) password_hash: String,
    pub(crate) admin: bool,
}

impl UserRow {
    async fn fetch(
        db: &SqlitePool,
        cookies: Cookies,
        header: Option<TypedHeader<Authorization<Bearer>>>,
    ) -> Result<Self, CommonError> {
        let token = if let Some(TypedHeader(Authorization(bearer))) = header {
            bearer.token().to_string()
        } else {
            let cookie = cookies
                .get("access_token")
                .ok_or(CommonError::MissingCredentials("Missing access token"))?;
            cookie.value().to_string()
        };

        let user_id = crate::utils::auth::validate_jwt(&token)?.id;

        Ok(crate::crud::fetch_user_from(db, &UserIdent::Id(user_id))
            .await?
            .ok_or(CommonError::WrongCredentials)?)
    }
}

#[async_trait]
impl<B: Send> FromRequest<B> for UserRow {
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(db) = Extension::<SqlitePool>::from_request(req)
            .await
            .expect("`SqlitePool` extension is missing");
        let cookies = Cookies::from_request(req)
            .await
            .expect("`Cookies` not found");
        // Used in case there is an error. It's async so we can't /easily/ do it in map_err
        let OriginalUri(uri) = OriginalUri::from_request(req)
            .await
            .expect("It's Infallible");

        let header = TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .ok();

        let user = UserRow::fetch(&db, cookies, header).await;
        user.map_err(move |e| {
            if uri.path().starts_with("/api") {
                Error::ApiError(e)
            } else {
                Error::HtmlError(e)
            }
        })
    }
}

/// Ways to identify a user
pub(crate) enum UserIdent {
    Id(String),
    Username(String),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UserClaims {
    pub(crate) id: String,
}
