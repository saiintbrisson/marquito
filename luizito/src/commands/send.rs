use std::path::PathBuf;

use futures::stream::{self, StreamExt};
use reqwest::Client;
use tokio::fs;

use crate::utils::create_url;

pub async fn send_files(files: Vec<PathBuf>) -> reqwest::Result<()> {
    let iter = files.into_iter().map(send_file);

    let mut stream = stream::iter(iter).buffer_unordered(crate::MAX_PARALLEL_REQUESTS);
    while let Some(next) = stream.next().await {
        next?;
    }
    Ok(())
}

pub async fn send_file(file: PathBuf) -> reqwest::Result<()> {
    let file_contents = fs::read(&file).await.unwrap();

    let url = create_url(&file);
    let request = Client::new().post(url).body(file_contents);

    request.send().await.map(|_| ())
}
