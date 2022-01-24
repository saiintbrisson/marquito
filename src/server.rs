pub mod request;
pub mod response;

use std::{future::Future, io, net::SocketAddr};

pub use request::{recv, Request};
pub use response::{send, Response};
use tokio::net::TcpStream;

pub const LINE_DELIMITER: &[u8] = b"\r\n";
pub const REQUEST_DELIMITER: &[u8] = b"\r\n\r\n";

type Handler<F> = fn(Request) -> F;

pub struct Server<F> {
    address: String,
    handler: Handler<F>,
}

impl<F> Server<F>
where
    F: Future<Output = Response> + Send + Sync + 'static,
{
    pub fn new(address: impl ToString, handler: Handler<F>) -> Self {
        Self {
            address: address.to_string(),
            handler,
        }
    }

    pub async fn run(self) -> io::Result<()> {
        let Server { address, handler } = self;

        let listener = tokio::net::TcpListener::bind(address).await?;

        loop {
            let (socket, addr) = listener.accept().await?;
            tokio::spawn(async move {
                tokio::select! {
                    _ = Self::handle_request(socket, handler, addr) => {},
                    _ = tokio::time::sleep(crate::TIMEOUT_DURATION) => {},
                }
            });
        }
    }

    #[tracing::instrument(skip(socket, handler))]
    async fn handle_request(mut socket: TcpStream, handler: Handler<F>, addr: SocketAddr) {
        let req = match recv(&mut socket).await {
            Ok(request) => request,
            Err(err) => {
                tracing::warn!(%err, "failed to read request");
                return;
            }
        };
        tracing::debug!(?req, "received request");

        let resp = handler(req).await;
        tracing::debug!(?resp, "sending response");
        if let Err(err) = send(&mut socket, &resp).await {
            tracing::warn!(%err, "failed to send response");
        }
    }
}
