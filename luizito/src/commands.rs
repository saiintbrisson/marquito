use std::{path::PathBuf, io};
use futures::{stream::{Stream, FuturesUnordered, StreamExt}};
use reqwest::Client;


pub async fn get_files(files: Vec<PathBuf>, _save_files: bool) -> io::Result<()> {
    let files = files.into_iter().filter_map(|file| file.to_str().map(ToString::to_string));
    let mut futures: FuturesUnordered<_> = files.map(|file| get_file(file, _save_files)).collect();

    while let Some(next) = futures.next().await {
        let next = next?;
        let message = std::str::from_utf8(&next).unwrap_or("This file contains non-utf8");
        println!("{}", message);
    }

    todo!()
}

pub async fn get_file(file: String, _save_file: bool) -> io::Result<Bytes> {
    let client = Client::new();
    let request = client.get(file).send().await.unwrap();
    request.bytes()

}

pub async fn send_files(files: Vec<PathBuf>) -> io::Result<()> {
    todo!()
}