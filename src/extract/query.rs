use {
    axum::{
        async_trait,
        extract::{rejection::QueryRejection, FromRequest, RequestParts},
        http::StatusCode,
        BoxError,
    },
    serde::de::DeserializeOwned,
    serde_json::{json, Value},
};

pub(crate) struct Query<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Query<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::Json`
    T: DeserializeOwned,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, err_message) = match rejection {
                    QueryRejection::FailedToDeserializeQueryString(e) => {
                        (StatusCode::BAD_REQUEST, e.to_string())
                    }
                    err => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unknown internal error: {}", err),
                    ),
                };

                let body = axum::Json(json!({ "error": err_message }));
                Err((status, body))
            }
        }
    }
}
