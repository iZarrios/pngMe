use std::process;

use args::{Cli, Commands};
use clap::Parser;
use commands::run;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Err(error) = run(&cli) {
        eprintln!("An error occurred: {}", error);
        process::exit(1);
    }

    Ok(())
}

// fn list_files(path: &str) -> Result<Vec<String>> {
//     let files = std::fs::read_dir(path)?
//         .filter_map(|re| re.ok())
//         .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
//         .filter_map(|e| e.file_name().into_string().ok())
//         .collect();
//     Ok(files)
// }
