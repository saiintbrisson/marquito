use std::str::Utf8Error;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("failed to handle request {0}")]
    IoError(#[from] std::io::Error),
    #[error("invalid request {0}")]
    InvalidRequest(http::Error),
    #[error("invalid response {0}")]
    InvalidResponse(http::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("failed to handle request {0}")]
    IoError(#[from] std::io::Error),
    #[error("invalid encoding {0}")]
    InvalidEncoding(#[from] Utf8Error),
    #[error("invalid format")]
    InvalidFormat,
    #[error("http error {0}")]
    HttpError(#[from] http::Error),
    #[error("unsupported version")]
    UnsupportedVersion,
}

// impl From<ToStrError> for RequestError {
//     fn from(_: ToStrError) -> Self {
//         Self::InvalidEncoding
//     }
// }
