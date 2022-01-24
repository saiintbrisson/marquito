mod error;
mod handler;
mod server;

use std::process;

use server::Server;
use tracing::Level;

const FILE_DIRECTORY: &str = "files";
const SERVER_ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    if let Err(err) = tokio::fs::create_dir_all(FILE_DIRECTORY).await {
        tracing::error!(%err, "failed to create ./{FILE_DIRECTORY} directory");
        process::exit(1);
    }

    let server = Server::new(SERVER_ADDRESS, handler::handle);

    if let Err(err) = server.run().await {
        tracing::error!(%err, "server failed");
        process::exit(1);
    }
}
