use anyhow::Result;
use proc_macro2::Span;
use proc_macro_error::abort;

use super::Value;

pub(crate) fn call_macro(name: &str, _args: &[Box<dyn Value>], span: Span) -> Result<Box<dyn Value>> {

	if cfg!(feature = "nested") {
		abort!(span,
			"Macro `{}!` cannot be called inside `build_info::format!`", name;
			note = "Only macros using proc-macro-nested are available for use in `build_info::format!`, as of now.";
			note = "The `nested` feature is enabled, but did not help.";
		)
	} else {
		abort!(span,
			"Macro `{}!` cannot be called inside `build_info::format!`", name;
			note = "No macros are implemented for use in `build_info::format!`, as of now.";
			note = "The `nested` feature might help.";
		)
	}
}
