use std::borrow::Cow;

use {
    axum::{
        http::{StatusCode, Uri},
        response::{IntoResponse, Redirect, Response},
        Json,
    },
    serde_json::json,
};

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    /// An error that will return a JSON response to the user
    ApiError(CommonError),
    #[error(transparent)]
    /// An error that will return Html or a redirect to the user
    HtmlError(CommonError),
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum CommonError {
    #[error("Resource not found")]
    NotFound,
    #[error("{0}")]
    MissingCredentials(&'static str),
    #[error(transparent)]
    InvalidCredentials(#[from] jwt_simple::Error),
    #[error("Wrong credentials provided")]
    WrongCredentials,
    #[error("Database error")]
    Db {
        msg: Option<Cow<'static, str>>,
        source: sqlx::Error,
    },
    #[error("{msg}")]
    Other {
        msg: Cow<'static, str>,
        code: StatusCode,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ApiError(err) => {
                let (code, err_msg) = match err {
                    CommonError::NotFound => {
                        (StatusCode::NOT_FOUND, Cow::Borrowed("Resource not found"))
                    }
                    CommonError::MissingCredentials(e) => {
                        (StatusCode::BAD_REQUEST, Cow::Borrowed(e))
                    }
                    CommonError::InvalidCredentials(e) => {
                        (StatusCode::BAD_REQUEST, Cow::Owned(format!("{}", e)))
                    }
                    CommonError::WrongCredentials => (
                        StatusCode::BAD_REQUEST,
                        Cow::Borrowed("Wrong credentials provided"),
                    ),
                    CommonError::Db { msg, source } => {
                        tracing::error!("Db Error: {:?}", source);
                        let code = if let Some(e) = source.as_database_error() {
                            if e.message().starts_with("UNIQUE constraint failed") {
                                StatusCode::BAD_REQUEST
                            } else {
                                StatusCode::INTERNAL_SERVER_ERROR
                            }
                        } else {
                            StatusCode::INTERNAL_SERVER_ERROR
                        };

                        let msg = match msg {
                            Some(msg) => msg,
                            None => Cow::Borrowed("Database error"),
                        };

                        (code, msg)
                    }
                    CommonError::Other { code, msg } => (code, msg),
                };

                let body = Json(json!({ "error": err_msg }));
                (code, body).into_response()
            }
            Error::HtmlError(err) => {
                if matches!(
                    err,
                    CommonError::MissingCredentials(_)
                        | CommonError::WrongCredentials
                        | CommonError::InvalidCredentials(_)
                ) {
                    Redirect::to(Uri::from_static("/login")).into_response()
                } else {
                    Redirect::to(Uri::from_static("/505")).into_response()
                }
            }
        }
    }
}

impl From<(StatusCode, &'static str)> for CommonError {
    fn from((code, msg): (StatusCode, &'static str)) -> Self {
        Self::Other {
            code,
            msg: Cow::Borrowed(msg),
        }
    }
}

impl From<(StatusCode, String)> for CommonError {
    fn from((code, msg): (StatusCode, String)) -> Self {
        Self::Other {
            code,
            msg: Cow::Owned(msg),
        }
    }
}

impl From<sqlx::Error> for CommonError {
    fn from(e: sqlx::Error) -> Self {
        Self::Db {
            msg: None,
            source: e,
        }
    }
}
