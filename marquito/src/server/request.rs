use std::str::from_utf8;

use bytes::{Buf, BytesMut};
use http::{header::CONTENT_LENGTH, request::Builder, Error, Method, Uri, Version};
use memchr::memmem;
use once_cell::sync::Lazy;
use tokio::{io::AsyncReadExt, net::TcpStream};

use super::{LINE_DELIMITER, REQUEST_DELIMITER};
use crate::error::RequestError;

pub type Request = http::Request<Option<bytes::Bytes>>;

static FINDER: Lazy<memmem::Finder> = Lazy::new(|| memmem::Finder::new(LINE_DELIMITER));

pub async fn recv(socket: &mut TcpStream) -> Result<Request, RequestError> {
    let mut buf = BytesMut::with_capacity(1024);

    let position = loop {
        socket.read_buf(&mut buf).await?;

        if let Some(position) = memmem::find(&buf, REQUEST_DELIMITER) {
            break position;
        }
    };

    let request = buf.split_to(position).freeze();
    let request = from_slice(&request)?;
    buf.advance(REQUEST_DELIMITER.len());

    let content_length = request
        .headers_ref()
        .and_then(|map| map.get(CONTENT_LENGTH));

    match content_length {
        Some(content_length) => {
            let content_lenght = content_length.to_str()?.parse::<usize>()?;

            if buf.len() < content_lenght {
                buf.resize(content_lenght, 0);
                socket.read_exact(&mut buf).await?;
            }

            request
                .body(Some(buf.freeze()))
                .map_err(RequestError::HttpError)
        }
        None => request.body(None).map_err(RequestError::HttpError),
    }
}

fn from_slice(buf: &[u8]) -> Result<Builder, RequestError> {
    let mut buf = from_utf8(buf)?;
    let mut request_line = split_to_delimiter(&mut buf)?;

    //request line = "METHOD PATH HTTP/VERSION\r\n"
    let method = split_to_byte(&mut request_line, b' ')?;
    let path = split_to_byte(&mut request_line, b' ')?;
    let version = request_line;

    let mut builder = http::Request::builder()
        .method(Method::try_from(method).map_err(Error::from)?)
        .uri(Uri::try_from(path).map_err(Error::from)?)
        .version(match version {
            "HTTP/0.9" => Version::HTTP_09,
            "HTTP/1.0" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            "HTTP/2.0" => Version::HTTP_2,
            "HTTP/3.0" => Version::HTTP_3,
            _ => return Err(RequestError::UnsupportedVersion),
        });

    // header = "Name: Value\r\n"
    while let Ok(mut header) = split_to_delimiter(&mut buf) {
        let key = split_to_byte(&mut header, b':')?;
        builder = builder.header(key, header.trim_start());
    }

    Ok(builder)
}

fn split_to_byte<'a>(buf: &mut &'a str, byte: u8) -> Result<&'a str, RequestError> {
    memchr::memchr(byte, buf.as_bytes())
        .map(|e| {
            let part = &buf[..e];
            *buf = &buf[e + 1..];
            part
        })
        .ok_or(RequestError::InvalidFormat)
}

fn split_to_delimiter<'a>(buf: &mut &'a str) -> Result<&'a str, RequestError> {
    if buf.len() == 0 {
        return Err(RequestError::InvalidFormat);
    }

    match FINDER.find(buf.as_bytes()) {
        Some(pos) => {
            let part = &buf[..pos];
            *buf = &buf[pos + LINE_DELIMITER.len()..];
            Ok(part)
        }
        None => {
            let part = &buf[..];
            *buf = &buf[part.len()..];
            Ok(part)
        }
    }
}
