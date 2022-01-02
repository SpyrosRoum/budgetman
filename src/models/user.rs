use {
    axum::{
        async_trait,
        extract::{FromRequest, OriginalUri, RequestParts, TypedHeader},
    },
    headers::{authorization::Bearer, Authorization},
    sea_query::{self, Iden},
    serde::{Deserialize, Serialize},
    sqlx::PgPool,
    tower_cookies::Cookies,
    uuid::Uuid,
};

use crate::{crud, CommonError, Error};

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
    pub(crate) id: uuid::Uuid,
    pub(crate) username: String,
    pub(crate) password_hash: String,
    pub(crate) admin: bool,
}

fn extract_token(
    cookies: Cookies,
    header: Option<TypedHeader<Authorization<Bearer>>>,
) -> Result<String, CommonError> {
    if let Some(TypedHeader(Authorization(bearer))) = header {
        Ok(bearer.token().to_string())
    } else {
        let cookie = cookies
            .get("access_token")
            .ok_or(CommonError::MissingCredentials("Missing access token"))?;
        Ok(cookie.value().to_string())
    }
}

/// Ways to identify a user
pub(crate) enum UserIdent {
    Id(Uuid),
    Username(String),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UserClaims {
    pub(crate) id: Uuid,
    pub(crate) username: String,
}

impl UserClaims {
    #[allow(dead_code)]
    pub(crate) async fn fetch_user(&self, db: &PgPool) -> Result<UserRow, CommonError> {
        crud::fetch_user_from(db, &UserIdent::Id(self.id))
            .await?
            .ok_or(CommonError::WrongCredentials)
    }
}

#[async_trait]
impl<B: Send> FromRequest<B> for UserClaims {
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request(req)
            .await
            .expect("`Cookies` not found");
        // Used in case there is an error. It's async so we can't /easily/ do it in map_err
        let OriginalUri(uri) = OriginalUri::from_request(req)
            .await
            .expect("It's Infallible");
        let err_type = if uri.path().starts_with("/api") {
            Error::ApiError
        } else {
            Error::HtmlError
        };
        let header = TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .ok();

        let token = extract_token(cookies, header).map_err(err_type)?;

        crate::utils::auth::validate_jwt(&token).map_err(err_type)
    }
}
