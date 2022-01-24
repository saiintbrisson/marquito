use std::str::Utf8Error;

#[derive(Debug, thiserror::Error)]
pub enum ResponseError {
    #[error("failed to write response {0}")]
    IoError(#[from] std::io::Error),
    #[error("failed to format response {0}")]
    FmtError(#[from] std::fmt::Error),
    #[error("failed to format response {0}")]
    InvalidHeaderEncoding(#[from] http::header::ToStrError),
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
    #[error("invalid header value encoding {0}")]
    InvalidHeaderEncoding(#[from] http::header::ToStrError),
    #[error("invalid content length value {0}")]
    InvalidContentLength(#[from] std::num::ParseIntError),
}
