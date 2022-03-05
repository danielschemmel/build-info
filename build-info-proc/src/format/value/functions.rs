use anyhow::Result;
use proc_macro2::Span;
use proc_macro_error::abort;

use super::Value;

pub(crate) fn call_function(name: &str, _args: &[Box<dyn Value>], span: Span) -> Result<Box<dyn Value>> {
	abort!(span,
		"Function `{}` cannot be called inside `build_info::format!`", name;
		note = "No (non-macro) functions are implemented for use in `build_info::format!`, as of now.";
	)
}
