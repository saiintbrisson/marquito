mod cli;
mod commands;

use cli::SubCommand;
use std::process;

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
