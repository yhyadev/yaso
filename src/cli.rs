use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Run a specific script")]
    Run { file_path: PathBuf },
}
