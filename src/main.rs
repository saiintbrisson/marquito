mod error;
mod request;
mod response;

use std::{io, net::SocketAddr, process, str::from_utf8, fmt::Write as _};

use bytes::{Buf, BytesMut};
use error::{RequestError};
use http::header::CONTENT_LENGTH;
use memchr::memmem;
use request::Request;
use response::Response;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    // :80 - reservada, só roda no root
    let address = "127.0.0.1:8080";
    let result = Server::new(address).run().await;

    result.unwrap_or_else(|err| {
        eprintln!("{:#?}", err);
        process::exit(1);
    });
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: impl ToString) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    pub async fn run(self) -> io::Result<()> {
        let Server { address } = self;

        let listener = tokio::net::TcpListener::bind(address).await?;

        loop {
            let (socket, addr) = listener.accept().await?;
            tokio::spawn(handle_request(socket, addr));
        }
    }
}

#[tracing::instrument(skip(socket))]
async fn handle_request(mut socket: TcpStream, addr: SocketAddr) {
    let request = recv(&mut socket).await.unwrap_or_else(|_err| todo!());
    tracing::debug!("received request");

    let response = generate_response(request).await;
    send_response(&mut socket, response).await;
}

async fn send_response(socket: &mut TcpStream, response: Response) -> io::Result<()> {
    let mut buf = BytesMut::with_capacity(1024);

    write!(buf, "{:?} {:?}\r\n", response.version(), response.status()).unwrap();
    for (key, value) in response.headers() {
        write!(buf, "{}: {}\r\n", key, value.to_str().unwrap()).unwrap();
    }
    write!(buf, "\r\n").unwrap();

    if let Some(body) = response.body() {
        buf.extend_from_slice(body.as_slice());
    }

    socket.write_all_buf(&mut buf).await
}

async fn generate_response(request: Request) -> Response {
    use http::method::Method;

    fn skip_one_char(text: &str) -> &str {
        let mut chars = text.chars();
        let _ = chars.next();
        chars.as_str()
    }

    let uri = skip_one_char(request.uri().path()).to_owned();

    match request.method().to_owned() {
        Method::GET => response::get_response(uri).await,
        Method::POST => response::post_response(uri, request.into_body()).await,
        _ => unreachable!("Unsupported method"),
    }
}

const LINE_DELIMITER: &[u8] = b"\r\n";
const REQUEST_DELIMITER: &[u8] = b"\r\n\r\n";

async fn recv(socket: &mut TcpStream) -> Result<Request, RequestError> {
    let mut buf = BytesMut::with_capacity(1024);

    let mut position;
    loop {
        socket.read_buf(&mut buf).await?;
        
        position = memmem::find(&buf, REQUEST_DELIMITER);
        if position.is_some() {
            break;
        }
    }

    let request = buf.split_to(position.unwrap()).freeze();
    let request = request::from_slice(&request)?;
    buf.advance(REQUEST_DELIMITER.len());

    let content_length = request
        .headers_ref()
        .and_then(|map| map.get(CONTENT_LENGTH));

    if let Some(content_lenght) = content_length {
        let content_lenght = from_utf8(content_lenght.as_bytes())?.parse::<usize>().unwrap();

        if buf.len() < content_lenght {
            buf.resize(content_lenght, 0);
            socket.read_exact(&mut buf).await?;
        }

        request
            .body(Some(buf.freeze()))
            .map_err(RequestError::HttpError)
    } else {
        request.body(None).map_err(RequestError::HttpError)
    }
}
