use anyhow::Result;
use proc_macro2::Span;
use proc_macro_error::{abort, abort_if_dirty, emit_error};

use super::{as_arguments_1, Value};

pub(crate) fn call_macro(name: &str, args: &[Box<dyn Value>], span: Span) -> Result<Box<dyn Value>> {
	match name {
		"concat" => {
			let mut result = String::new();
			for (i, value) in args.iter().enumerate() {
				if let Some(value) = value.as_any().downcast_ref::<String>() {
					result += value;
				} else {
					emit_error!(span,
						"Argument {} to `concat!` is no string.", i;
						note = "Try using `.to_string()`.";
						note = "Using `concat!` inside `build_info::format!` is special.";
					);
				}
			}
			abort_if_dirty();
			Ok(Box::new(result))
		}
		"env" => {
			let (name,) = as_arguments_1::<String>(args)?;
			let value = std::env::var(name).unwrap_or_else(|_| abort!(span, "Environment variable `{}` not defined.", name));
			Ok(Box::new(value))
		}
		"option_env" => {
			let (name,) = as_arguments_1::<String>(args)?;
			let value = std::env::var(name).ok();
			Ok(Box::new(value))
		}
		_ => {
			if cfg!(feature = "nested") {
				abort!(span,
					"Macro `{}!` cannot be called inside `build_info::format!`", name;
					note = "Only macros using proc-macro-nested, `concat!`, `env!` and `option_env!` are available for use in `build_info::format!`, as of now.";
					note = "The `nested` feature is enabled, but did not help.";
				)
			} else {
				abort!(span,
					"Macro `{}!` cannot be called inside `build_info::format!`", name;
					note = "Only `concat!`, `env!` and `option_env!` are implemented for use in `build_info::format!`, as of now.";
					note = "Depending on the macro, the `nested` feature might help.";
				)
			}
		}
	}
}
