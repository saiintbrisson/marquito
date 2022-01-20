// pub struct ParseError;
// pub struct Req;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("failed to handle request {0}")]
    IoError(#[from] std::io::Error),
    #[error("invalid method {0}")]
    InvalidMethod(String),
    #[error("path {0} not found")]
    NotFound(String),
}
