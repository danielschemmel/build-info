use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

mod expression;
mod format;
mod function;

#[proc_macro]
pub fn versionator(input: TokenStream) -> TokenStream {
	function::versionator(input)
}

#[proc_macro_hack]
pub fn version(input: TokenStream) -> TokenStream {
	expression::version(input)
}

#[proc_macro_hack]
pub fn format(input: TokenStream) -> TokenStream {
	format::format(input)
}
