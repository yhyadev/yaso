use rquickjs::loader::{FileResolver, ScriptLoader};
use rquickjs::{CatchResultExt, CaughtError, Context, Ctx, Module, Object, Runtime};

use std::path::PathBuf;
use std::process::exit;

pub struct VirtualMachine {
    context: Context,
    runtime: Runtime,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        let resolver = (FileResolver::default()
            .with_path(".")
            .with_pattern("{}.cjs")
            .with_pattern("{}.mjs"),);

        let loader = (ScriptLoader::default()
            .with_extension("mjs")
            .with_extension("cjs"),);

        let runtime = Runtime::new().expect("failed to create a Runtime");

        runtime.set_loader(resolver, loader);

        let context = Context::full(&runtime).expect("failed to create a Context");

        VirtualMachine { context, runtime }
    }

    fn load_module<'js>(
        ctx: &Ctx<'js>,
        file_path: &PathBuf,
    ) -> Result<Object<'js>, rquickjs::Error> {
        Module::import(ctx, file_path.to_string_lossy().to_string())
    }

    pub fn run_module(&self, file_path: &PathBuf) {
        self.context.with(|ctx| {
            VirtualMachine::load_module(&ctx, file_path)
                .catch(&ctx)
                .unwrap_or_else(|err| VirtualMachine::print_error_and_exit(err));
        });
    }

    fn print_error_and_exit<'js>(err: CaughtError<'js>) -> ! {
        let mut error_message = String::new();

        match err {
            CaughtError::Error(err) => {
                error_message = err.to_string();
            }

            CaughtError::Exception(exception) => {
                if let Some(message) = exception.message() {
                    error_message.push_str(&message);
                    error_message.push('\n');
                }

                if let Some(stack) = exception.stack() {
                    error_message.push_str(&stack);
                }
            }

            // This one needs a console implementation
            CaughtError::Value(_) => todo!(),
        }

        eprintln!("{}", error_message);

        exit(1);
    }
}
