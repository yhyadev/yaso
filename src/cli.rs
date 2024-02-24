use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Parser)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Run a specific script")]
    Run {
        file_path: PathBuf,
        args: Vec<String>,
    },
}

pub fn get_program_argv() -> Vec<String> {
    let mut argv = Vec::new();

    match CLI::parse().command {
        Command::Run { file_path, args } => {
            argv.push(file_path.to_string_lossy().to_string());
            argv.extend(args);
        }
    }

    argv
}
