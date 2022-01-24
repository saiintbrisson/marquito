use std::{path::PathBuf, str};

use bytes::Bytes;
use futures::stream::{self, StreamExt};
use tokio::{fs, io::AsyncWriteExt};

use crate::utils::create_url;

pub async fn get_files(files: Vec<PathBuf>, _save_files: bool) -> reqwest::Result<()> {
    let iter = files.into_iter().map(|file| get_file(file, _save_files));
    let mut stream = stream::iter(iter).buffer_unordered(crate::MAX_PARALLEL_REQUESTS);

    while let Some(file_contents) = stream.next().await {
        let file_contents = file_contents?;
        let message = str::from_utf8(&file_contents).unwrap_or("This file contains non-utf8");
        println!("{}", message);
    }

    Ok(())
}

pub async fn get_file(file: PathBuf, save_file: bool) -> reqwest::Result<Bytes> {
    let url = create_url(&file);
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    if save_file {
        let mut file = fs::File::create(&file).await.unwrap();
        file.write_all(&bytes).await.unwrap();
    }

    Ok(bytes)
}
