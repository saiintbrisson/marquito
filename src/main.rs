use std::{collections::HashMap, io, net::SocketAddr, process, str::FromStr};

use bytes::{Buf, Bytes, BytesMut};
use error::RequestError;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

mod error;

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

async fn handle_request(mut socket: TcpStream, addr: SocketAddr) {
    let request = recv(&mut socket)
        .await
        .unwrap_or_else(|_err| panic!("eitakkk"));

    socket
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 20\r\n\r\nbom dia kkkkkkkkkkkk")
        .await
        .unwrap();

    let response = generate_response(request);
    // send_response(response);
}

struct Response {
    code: u16,
    headers: Headers,
}

async fn generate_response(request: Request) -> Response {
    let Request { method, .. } = request;

    match method {
        Method::Get => {}
        Method::Post => {}
    }
    todo!()
}

async fn recv(socket: &mut TcpStream) -> Result<Request, RequestError> {
    let mut buf = BytesMut::with_capacity(1024);

    let mut position;
    loop {
        socket.read_buf(&mut buf).await?;

        position = buf.windows(4).position(|x| x == REQUEST_DELIMITER);

        if position.is_some() {
            break;
        }
    }

    let request_header = buf.split_to(position.unwrap()).freeze();
    let mut request = Request::from(&request_header[..]);
    buf.advance(REQUEST_DELIMITER.len());

    if let Some(content_lenght) = request.headers.get("Content-Lenght") {
        let content_lenght = content_lenght.parse::<usize>().unwrap();

        while buf.len() < content_lenght {
            socket.read_buf(&mut buf).await?;
        }

        request.body = Some(buf.freeze());
    }

    Ok(request)
}

type Headers = HashMap<String, String>;

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    version: String,
    headers: Headers,
    body: Option<Bytes>,
}

impl Request {
    pub fn from(slice: &[u8]) -> Self {
        let request_line_length = slice.windows(2).position(|x| x == LINE_DELIMITER).unwrap();

        let str = std::str::from_utf8(slice).unwrap();

        //request line = "METHOD PATH HTTP/VERSION\r\n"
        let (request_line, rest) = str.split_at(request_line_length);
        let mut request_line = request_line.trim_end().splitn(3, |x| x == ' ');
        let method = request_line.next().unwrap();
        let path = request_line.next().unwrap();
        let version = request_line.next().unwrap();

        let rest = rest.trim();
        // header = "Name: Value\r\n"

        let mut map = HashMap::<String, String>::new();
        for line in rest.lines() {
            let (key, value) = line.split_once(":").unwrap();
            map.insert(key.into(), value.trim_start().into());
        }

        Request {
            method: method.parse().unwrap(),
            path: path.to_string(),
            version: version.to_string(),
            headers: map,
            body: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

impl FromStr for Method {
    type Err = error::RequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err(error::RequestError::InvalidMethod(s.to_string())),
        }
    }
}

// GET / HTTP/1.1\r\n
// Host: localhost:8080\r\n
// \r\n

// GET / HTTP/1.1\r\n
// Content-Length: 12\r\n
// Host: localhost:8080\r\n
// \r\n
// Hello World!

const LINE_DELIMITER: &[u8] = b"\r\n";
const REQUEST_DELIMITER: &[u8] = b"\r\n\r\n";
