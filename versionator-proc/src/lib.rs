use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

mod format;
mod function;
mod init_value;

#[proc_macro]
pub fn versionator(input: TokenStream) -> TokenStream {
	function::versionator(input)
}

#[proc_macro_hack]
pub fn format(input: TokenStream) -> TokenStream {
	format::format(input)
}
