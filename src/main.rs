mod error;
mod handler;
mod server;

use std::{process, time::Duration};

use server::Server;
use tracing::Level;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const TIMEOUT_DURATION: Duration = Duration::from_secs(5);
// Directory where files are stored and read from
const FILES_STORAGE_DIRECTORY: &str = "files/";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    if let Err(err) = tokio::fs::create_dir_all(FILES_STORAGE_DIRECTORY).await {
        tracing::error!(%err, "failed to create ./{FILES_STORAGE_DIRECTORY} directory");
        process::exit(1);
    }

    let server = Server::new(SERVER_ADDRESS, handler::handle);

    if let Err(err) = server.run().await {
        tracing::error!(%err, "server failed");
        process::exit(1);
    }
}
