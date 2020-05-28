use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

mod format;
#[cfg(feature = "runtime")]
mod function;

/**
Call as `build_info!(fn name)` to create a function called `name` that returns a reference to a lazily created
and cached `BuildInfo` object.

This macro also accepts a visibility specifier for the generated function, such as `build_info!(pub fn version)`.
*/
#[cfg(feature = "runtime")]
#[proc_macro]
pub fn build_info(input: TokenStream) -> TokenStream {
	function::build_info(input)
}

#[proc_macro_hack]
pub fn format(input: TokenStream) -> TokenStream {
	format::format(input)
}
