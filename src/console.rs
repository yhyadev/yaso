use rquickjs::function::{Func, Rest};
use rquickjs::{Ctx, Object, Result as QuickJsResult, Type, Value};

use std::io::{stderr, stdout, Write};

pub fn init(ctx: &Ctx<'_>) -> QuickJsResult<()> {
    let globals = ctx.globals();

    let console = Object::new(ctx.clone())?;

    console.set("log", Func::from(log_stdout))?;
    console.set("info", Func::from(log_stdout))?;
    console.set("trace", Func::from(log_stdout))?;
    console.set("debug", Func::from(log_stdout))?;
    console.set("error", Func::from(log_stderr))?;
    console.set("warn", Func::from(log_stderr))?;
    console.set("assert", Func::from(log_assert))?;

    globals.set("console", console)?;

    Ok(())
}

fn log_assert(expression: bool, args: Rest<Value<'_>>) -> QuickJsResult<()> {
    if !expression {
        log_stderr(args)?;
    }

    Ok(())
}

fn log_stdout(args: Rest<Value<'_>>) -> QuickJsResult<()> {
    log_write(stdout(), args)
}

fn log_stderr(args: Rest<Value<'_>>) -> QuickJsResult<()> {
    log_write(stderr(), args)
}

fn js_format(args: Rest<Value<'_>>) -> QuickJsResult<String> {
    let mut result = String::new();

    for arg in args.iter() {
        result.push_str(js_stringify(arg)?.as_str());
        result.push(' ');
    }

    Ok(result)
}

pub fn js_stringify(arg: &Value<'_>) -> QuickJsResult<String> {
    let mut result = String::new();

    match arg.type_of() {
        Type::String => result = arg.as_string().unwrap().to_string()?,

        Type::Bool => result = arg.as_bool().unwrap().to_string(),

        Type::Int => result = arg.as_int().unwrap().to_string(),

        Type::BigInt => {
            result = arg.as_big_int().unwrap().clone().to_i64()?.to_string();

            result.push('n');
        },

        Type::Float => result = arg.as_float().unwrap().to_string(),

        Type::Array => {
            let array = arg.as_array().unwrap();

            result.push('[');

            for (i, value) in array.clone().into_iter().enumerate() {
                result.push_str(js_stringify(&value?)?.as_str());

                if i < array.len() - 1 {
                    result.push_str(", ");
                }
            }

            result.push(']');
        }

        Type::Symbol => {
            let description = arg.as_symbol().unwrap().description()?;
            let description = description.to_string()?;

            result.push_str("Symbol(");

            if description != "undefined" {
                result.push_str(&description);
            }
            
            result.push(')');
        }

        Type::Exception => {
            let exception = arg.as_exception().unwrap();

            if let Some(message) = exception.message() {
                result.push_str(exception.get::<&str, String>("name")?.as_str());
                result.push_str(": ");
                result.push_str(&message);
                result.push('\n');
            }

            if let Some(stack) = exception.stack() {
                result.push_str(&stack);
            }
        }

        // TODO: stringify these properly
        Type::Object => result.push_str("[Object]"),
        Type::Module => result.push_str("[Module]"),
        Type::Function => result.push_str("[Function]"),
        Type::Constructor => result.push_str("[Constructor]"),

        Type::Uninitialized | Type::Undefined => result.push_str("undefined"),

        Type::Null => result.push_str("null"),

        Type::Unknown => result.push_str("{unknown}"),
    };

    Ok(result)
}

fn log_write<O>(mut output: O, args: Rest<Value<'_>>) -> QuickJsResult<()>
where
    O: Write,
{
    let _ = output.write_all(js_format(args)?.as_bytes());

    let _ = output.write(b"\n");

    Ok(())
}
