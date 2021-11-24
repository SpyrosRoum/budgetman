use warp::http::StatusCode;
use common::ErrorResponse;

/// Helper function to create [`ErrorResponse`]s on te fly,
/// Basically sorter version of [`ErrorResponse::new`]
pub(crate) fn error_response<S: Into<String>>(code: StatusCode, msg: S) -> ErrorResponse {
    ErrorResponse::new(code, msg.into())
}
