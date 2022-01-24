mod error;
mod handler;
mod server;

use std::{process, sync::Arc, time::Duration};

use server::Server;
use tracing::Level;

const SERVER_ADDRESS_ENVIRONMENT_VARIABLE: &str = "MARQUITO_ADDRESS";
const SERVER_ADDRESS: &str = "127.0.0.1:23400";

// Directory where files are stored and read from
const STORAGE_DIRECTORY_ENVIRONMENT_VARIABLE: &str = "MARQUITO_DIRECTORY";
const STORAGE_DIRECTORY: &str = "files/";

const TIMEOUT_DURATION: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let directory = std::env::var(STORAGE_DIRECTORY_ENVIRONMENT_VARIABLE);
    let directory = directory.unwrap_or_else(|_| STORAGE_DIRECTORY.to_string());

    if let Err(err) = tokio::fs::create_dir_all(&directory).await {
        tracing::error!(%err, "failed to create ./{STORAGE_DIRECTORY} directory");
        process::exit(1);
    }

    let address = std::env::var(SERVER_ADDRESS_ENVIRONMENT_VARIABLE);
    let address = address.unwrap_or_else(|_| SERVER_ADDRESS.to_string());

    let state = Arc::new(AppState { directory });
    let server = Server::new(address, state, handler::handle);

    if let Err(err) = server.run().await {
        tracing::error!(%err, "server failed");
        process::exit(1);
    }
}

#[derive(Debug)]
pub struct AppState {
    pub directory: String,
}
