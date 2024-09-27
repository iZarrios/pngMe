use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "pngme",
    version = "0.1.0",
    about = "A tool for working with PNG files"
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode {
        png_file: PathBuf,
        chunk_type: String,
        message: String,
    },
    Decode {
        png_file: PathBuf,
        chunk_type: String,
    },
    Remove {
        png_file: PathBuf,
        chunk_type: String,
    },

    Print {
        png_file: PathBuf,
    },
}
