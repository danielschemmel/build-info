use proc_macro_error2::abort;
use proc_macro2::Span;

use super::Value;

pub(crate) fn call_function(name: &str, _args: &[Box<dyn Value>], span: Span) -> anyhow::Result<Box<dyn Value>> {
	abort!(span,
		"Function `{}` cannot be called inside `build_info::format!`", name;
		note = "No (non-macro) functions are implemented for use in `build_info::format!`, as of now.";
	)
}
