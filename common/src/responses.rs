use {serde::Serialize, warp::http::StatusCode};

#[derive(Serialize, Debug, Clone)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl warp::reject::Reject for ErrorResponse {}

impl ErrorResponse {
    pub fn new<S: Into<String>>(code: StatusCode, msg: S) -> Self {
        Self {
            code: code.as_u16(),
            message: msg.into(),
        }
    }

    pub fn get_code(&self) -> u16 {
        self.code
    }
}
