use std::path::Path;

use bytes::Bytes;
use http::StatusCode;
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
};

pub type Response = http::Response<Option<Vec<u8>>>;

pub async fn get_response(path: impl AsRef<Path>) -> Response {
    let file_contents = fs::read(path.as_ref()).await;

    match file_contents {
        Ok(contents) => http::Response::builder()
            .status(StatusCode::OK.as_u16())
            .header("Content-Length", contents.len())
            .body(Some(contents))
            .unwrap(),
        Err(_) => http::Response::builder()
            .status(StatusCode::BAD_REQUEST.as_u16())
            .body(None)
            .unwrap(),
    }
}

pub async fn post_response(path: impl AsRef<Path>, file_contents: Option<Bytes>) -> Response {
    let file_contents = match file_contents {
        Some(contents) => contents,
        None => return http::Response::builder()
            .status(StatusCode::BAD_REQUEST.as_u16())
            .body(None)
            .unwrap(),
    };

    let result = write_to_file(path.as_ref(), &file_contents).await;

    let response = match result {
        Ok(_) => http::Response::builder()
            .status(StatusCode::OK.as_u16()),
        Err(_) => http::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
    };
    response.body(None).unwrap()
}

async fn write_to_file(path: &Path, file_contents: &[u8]) -> io::Result<()> {
    let file = fs::File::create(path).await?;
    let mut writer = io::BufWriter::new(file);

    writer.write_all(file_contents).await?;
    writer.flush().await
}
