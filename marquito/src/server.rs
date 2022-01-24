pub mod request;
pub mod response;

use std::{future::Future, io, net::SocketAddr};

pub use request::{recv, Request};
pub use response::{send, Response};
use tokio::net::TcpStream;

pub const LINE_DELIMITER: &[u8] = b"\r\n";
pub const REQUEST_DELIMITER: &[u8] = b"\r\n\r\n";

type Handler<A, F> = fn(Request, A) -> F;

pub struct Server<A, F> {
    address: String,
    app_state: A,
    handler: Handler<A, F>,
}

impl<A, F> Server<A, F>
where
    A: Clone + Send + Sync + 'static,
    F: Future<Output = Response> + Send + Sync + 'static,
{
    pub fn new(address: impl ToString, app_state: A, handler: Handler<A, F>) -> Self {
        Self {
            address: address.to_string(),
            app_state,
            handler,
        }
    }

    pub async fn run(self) -> io::Result<()> {
        let Server {
            address,
            app_state,
            handler,
        } = self;

        let listener = tokio::net::TcpListener::bind(address).await?;
        let addr = listener.local_addr()?;
        tracing::info!(%addr, "server is running");

        loop {
            let (socket, addr) = listener.accept().await?;
            let app_state = app_state.clone();

            tokio::spawn(async move {
                tokio::select! {
                    _ = Self::handle_request(socket, app_state, handler, addr) => {},
                    _ = tokio::time::sleep(crate::TIMEOUT_DURATION) => {},
                }
            });
        }
    }

    #[tracing::instrument(skip(socket, app_state, handler))]
    async fn handle_request(
        mut socket: TcpStream,
        app_state: A,
        handler: Handler<A, F>,
        addr: SocketAddr,
    ) {
        let req = match recv(&mut socket).await {
            Ok(request) => request,
            Err(err) => {
                tracing::warn!(%err, "failed to read request");
                return;
            }
        };
        tracing::debug!(?req, "received request");

        let resp = handler(req, app_state).await;
        tracing::debug!(?resp, "sending response");
        if let Err(err) = send(&mut socket, &resp).await {
            tracing::warn!(%err, "failed to send response");
        }
    }
}
