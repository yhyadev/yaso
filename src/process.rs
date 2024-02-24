use std::collections::HashMap;

use crate::utils::export_default;

use rquickjs::function::Func;
use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Result as QuickJsResult};

pub fn cwd() -> String {
    std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

pub fn get_arch() -> &'static str {
    std::env::consts::ARCH
}

pub fn get_platform() -> &'static str {
    std::env::consts::OS
}

pub struct ProcessModule;

impl ModuleDef for ProcessModule {
    fn declare(declare: &mut Declarations) -> QuickJsResult<()> {
        declare.declare("argv")?;
        declare.declare("env")?;
        declare.declare("cwd")?;
        declare.declare("arch")?;
        declare.declare("platform")?;
        declare.declare("exit")?;
        declare.declare("default")?;

        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> QuickJsResult<()> {
        let argv = crate::cli::get_program_argv();

        let env: HashMap<String, String> = std::env::vars().collect();

        export_default(ctx, exports, |default| {
            default.set("argv", argv)?;
            default.set("env", env)?;
            default.set("cwd", Func::from(cwd))?;
            default.set("arch", get_arch())?;
            default.set("platform", get_platform())?;
            default.set(
                "exit",
                Func::from(|status_code: i32| std::process::exit(status_code)),
            )?;

            Ok(())
        })
    }
}
