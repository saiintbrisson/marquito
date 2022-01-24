use std::{path::Path, sync::Arc};

use bytes::Bytes;
use http::StatusCode;
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
};

use crate::{
    server::{response::IntoResponse, Request, Response},
    AppState,
};

pub async fn handle(request: Request, app_state: Arc<AppState>) -> Response {
    use http::method::Method;

    let uri = request.uri().path();

    if uri.len() < 2 {
        return (StatusCode::BAD_REQUEST, "Missing file name").into_response();
    }

    if !uri.starts_with('/') {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let uri = &uri[1..];
    if memchr::memchr2(b'/', b'\\', uri.as_bytes()).is_some() {
        return (
            StatusCode::BAD_REQUEST,
            "File name cannot contain '/' nor '\\'",
        )
            .into_response();
    }

    let uri = Path::new(&app_state.directory).join(uri);

    match request.method().to_owned() {
        Method::GET => get_response(uri).await,
        Method::POST => post_response(uri, request.into_body()).await,
        _ => {
            (
                StatusCode::NOT_FOUND,
                "Only GET and POST methods are accepted",
            )
                .into_response()
        }
    }
}

async fn get_response(path: impl AsRef<Path>) -> Response {
    let file_contents = fs::read(path.as_ref()).await;

    match file_contents {
        Ok(contents) => (StatusCode::OK, contents).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn post_response(path: impl AsRef<Path>, file_contents: Option<Bytes>) -> Response {
    let file_contents = match file_contents {
        Some(contents) => contents,
        None => {
            return (StatusCode::BAD_REQUEST, "Empty message").into_response();
        }
    };

    let result = write_to_file(path.as_ref(), &file_contents).await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
    .into_response()
}

async fn write_to_file(path: &Path, file_contents: &[u8]) -> io::Result<()> {
    let file = fs::File::create(path).await?;
    let mut writer = io::BufWriter::new(file);

    writer.write_all(file_contents).await?;
    writer.flush().await
}
