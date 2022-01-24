mod cli;
mod commands;
pub mod utils;

use std::process;

use cli::SubCommand;

const MAX_PARALLEL_REQUESTS: usize = 16;
const SERVER_BASE_URL: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    let args = cli::parse_args();

    let result = match args.cmd {
        SubCommand::Get { files, save } => commands::get_files(files, save).await,
        SubCommand::Send { files } => commands::send_files(files).await,
    };

    result.unwrap_or_else(|err| {
        eprintln!("{:#?}", err);
        process::exit(1);
    });
}
