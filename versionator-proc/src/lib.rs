use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

mod function;
mod expression;

#[proc_macro]
pub fn versionator(input: TokenStream) -> TokenStream {
	function::versionator(input)
}

#[proc_macro_hack]
pub fn version(input: TokenStream) -> TokenStream {
	expression::version(input)
}
