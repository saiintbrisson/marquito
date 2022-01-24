use std::path::PathBuf;

use clap::{Parser, ValueHint};

/// Luizito is a good person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Aqui eh a --help message
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

/// sample text
#[derive(Parser, PartialEq, Eq, Debug)]
pub enum SubCommand {
    /// Send one or more file.
    #[clap(alias = "s")]
    Send {
        /// Files to be sent.
        #[clap(required = true, min_values = 1, value_hint = ValueHint::AnyPath)]
        files: Vec<PathBuf>,
    },
    /// Get one or more file.
    #[clap(alias = "d")]
    Get {
        /// Files to be fetched.
        #[clap(required = true, min_values = 1, value_hint = ValueHint::AnyPath)]
        files: Vec<PathBuf>,

        /// Save files to disk.
        #[clap(short, long, env = "LUIZITO_SAVE_FILES")]
        save: bool,
    },
}

pub fn parse_args() -> Args {
    Args::parse()
}
