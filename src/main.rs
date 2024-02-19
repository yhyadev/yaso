use yaso::cli::{Arguments, Command};
use yaso::vm::VirtualMachine;

use clap::Parser;

use std::process::exit;

pub fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Run { file_path } => {
            if !file_path.exists() {
                eprintln!("{}: No such file or directory", file_path.display());

                exit(1);
            }

            let vm = VirtualMachine::new();

            vm.init();

            vm.run_module(&file_path);
        }
    };
}
