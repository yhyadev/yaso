use quick_js::Context;
use quick_js::ExecutionError;

use std::process::exit;

pub struct Runtime {
    context: Context,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            context: Context::new().unwrap(),
        }
    }

    pub fn run(&self, script: String) {
        self.context.eval(&script).unwrap_or_else(|err| {
            match err {
                ExecutionError::Conversion(conversion_err) => {
                    eprintln!("ConversionError: {}", conversion_err.to_string());
                },

                ExecutionError::Internal(internal_err) => {
                    eprintln!("InternalError: {}", internal_err);
                },

                ExecutionError::Exception(exception) => {
                    eprintln!("UncaughtException: {}", exception.into_string().unwrap());
                },

                ExecutionError::OutOfMemory => {
                    eprintln!("OutOfMemory");
                }

                _ => (),
            };

            exit(1);
        });
    }
}
