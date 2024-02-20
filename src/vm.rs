use rquickjs::loader::{FileResolver, ScriptLoader};
use rquickjs::{CatchResultExt, CaughtError, Context, Ctx, Module, Object, Runtime};

use std::path::Path;
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

    pub fn init(&self) {
        self.context.with(|ctx| {
            crate::console::init(&ctx)
                .catch(&ctx)
                .unwrap_or_else(|err| VirtualMachine::print_error_and_exit(ctx, err));
        })
    }

    fn load_module<'js>(ctx: &Ctx<'js>, file_path: &Path) -> Result<Object<'js>, rquickjs::Error> {
        Module::import(ctx, file_path.to_string_lossy().to_string())
    }

    pub fn run_module(&self, file_path: &Path) {
        self.context.with(|ctx| {
            VirtualMachine::load_module(&ctx, file_path)
                .catch(&ctx)
                .unwrap_or_else(|err| VirtualMachine::print_error_and_exit(ctx, err));
        });
    }

    fn print_error_and_exit<'js>(ctx: Ctx<'js>, err: CaughtError<'js>) -> ! {
        let error_message = match err {
            CaughtError::Error(err) => err.to_string(),

            CaughtError::Exception(exception) => crate::console::js_stringify(exception.as_value())
                .catch(&ctx)
                .unwrap_or_else(|err| VirtualMachine::print_error_and_exit(ctx, err)),

            CaughtError::Value(value) => crate::console::js_stringify(&value)
                .catch(&ctx)
                .unwrap_or_else(|err| VirtualMachine::print_error_and_exit(ctx, err)),
        };

        eprintln!("{}", error_message);

        exit(1);
    }
}
