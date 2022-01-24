use std::fmt::Write as _;

use bytes::{Bytes, BytesMut};
use http::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    HeaderValue, StatusCode,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::error::ResponseError;

pub type Response = http::Response<Option<Bytes>>;

pub trait IntoResponse {
    fn into_responese(self) -> Response;
}

impl IntoResponse for StatusCode {
    fn into_responese(self) -> Response {
        let mut response = http::Response::new(None);
        *response.status_mut() = self;
        response.headers_mut().insert(CONTENT_LENGTH, 0.into());

        response
    }
}

impl IntoResponse for Bytes {
    fn into_responese(self) -> Response {
        let body_len = self.len();
        let mut response = http::Response::new(Some(self));

        response.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_OCTET_STREAM.as_ref()),
        );
        response
            .headers_mut()
            .insert(CONTENT_LENGTH, body_len.into());

        response
    }
}

impl IntoResponse for Vec<u8> {
    fn into_responese(self) -> Response {
        IntoResponse::into_responese(Bytes::from(self))
    }
}

impl IntoResponse for &'static str {
    fn into_responese(self) -> Response {
        let body_len = self.len();
        let mut response = http::Response::new(Some(Bytes::from(self)));

        response.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
        );
        response
            .headers_mut()
            .insert(CONTENT_LENGTH, body_len.into());

        response
    }
}

impl<B: IntoResponse> IntoResponse for (StatusCode, B) {
    fn into_responese(self) -> Response {
        let mut response = self.1.into_responese();
        *response.status_mut() = self.0;

        response
    }
}

pub async fn send(socket: &mut TcpStream, response: &Response) -> Result<(), ResponseError> {
    let mut buf = BytesMut::with_capacity(1024);
    write!(buf, "{:?} {:?}\r\n", response.version(), response.status())?;

    for (key, value) in response.headers() {
        let value = value.to_str()?;
        write!(buf, "{}: {}\r\n", key, value)?;
    }

    write!(buf, "\r\n")?;

    if let Some(body) = response.body() {
        buf.extend_from_slice(&body);
    }

    socket.write_all(&buf).await.map_err(Into::into)
}
