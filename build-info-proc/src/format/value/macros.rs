use proc_macro_error2::{abort, abort_if_dirty, emit_error};
use proc_macro2::Span;

use super::{Value, as_named_arguments_1};

pub(crate) fn call_macro(
	name: &str,
	args: &[(Option<String>, Box<dyn Value>)],
	span: Span,
) -> anyhow::Result<Box<dyn Value>> {
	match name {
		"concat" => {
			let mut result = String::new();
			for (i, (name, value)) in args.iter().enumerate() {
				if name.is_some() {
					emit_error!(span, "`concat!` Takes no named arguments");
				}

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
			let (name,) = as_named_arguments_1::<String>(args)?;
			let value = std::env::var(name).unwrap_or_else(|_| abort!(span, "Environment variable `{}` not defined.", name));
			Ok(Box::new(value))
		}
		"option_env" => {
			let (name,) = as_named_arguments_1::<String>(args)?;
			let value = std::env::var(name).ok();
			Ok(Box::new(value))
		}
		_ => {
			abort!(span,
				"Macro `{}!` cannot be called inside `build_info::format!`", name;
				note = "Only `concat!`, `env!` and `option_env!` are implemented for use in `build_info::format!`, as of now.";
			)
		}
	}
}
