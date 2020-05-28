use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

mod format;
#[cfg(feature = "runtime")]
mod function;

#[cfg(feature = "runtime")]
#[proc_macro]
pub fn build_info(input: TokenStream) -> TokenStream {
	function::build_info(input)
}

#[proc_macro_hack]
pub fn format(input: TokenStream) -> TokenStream {
	format::format(input)
}
