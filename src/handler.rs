use std::path::Path;

use bytes::Bytes;
use http::StatusCode;
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
};

use crate::server::{response::IntoResponse, Request, Response};

pub async fn handle(request: Request) -> Response {
    use http::method::Method;

    fn skip_one_char(text: &str) -> &str {
        let mut chars = text.chars();
        let _ = chars.next();
        chars.as_str()
    }

    let uri = skip_one_char(request.uri().path()).to_owned();

    match request.method().to_owned() {
        Method::GET => get_response(uri).await,
        Method::POST => post_response(uri, request.into_body()).await,
        _ => unreachable!("Unsupported method"),
    }
}

async fn get_response(path: impl AsRef<Path>) -> Response {
    let file_contents = fs::read(path.as_ref()).await;

    match file_contents {
        Ok(contents) => (StatusCode::OK, contents).into_responese(),
        Err(_) => StatusCode::NOT_FOUND.into_responese(),
    }
}

async fn post_response(path: impl AsRef<Path>, file_contents: Option<Bytes>) -> Response {
    let file_contents = match file_contents {
        Some(contents) => contents,
        None => {
            return (StatusCode::BAD_REQUEST, "empty message").into_responese();
        }
    };

    let result = write_to_file(path.as_ref(), &file_contents).await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
    .into_responese()
}

async fn write_to_file(path: &Path, file_contents: &[u8]) -> io::Result<()> {
    let file = fs::File::create(path).await?;
    let mut writer = io::BufWriter::new(file);

    writer.write_all(file_contents).await?;
    writer.flush().await
}
