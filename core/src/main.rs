use yaso_core::cli::{Arguments, Command};
use yaso_core::runtime::Runtime;

use clap::Parser;

use std::process::exit;

pub fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Run { file_path } => {
            let file_content = std::fs::read_to_string(file_path.clone()).unwrap_or_else(|err| {
                eprintln!("{}: {}", file_path.display(), err);
                exit(1);
            });

            let runtime = Runtime::new();

            runtime.run(file_content);
        }
    };
}
