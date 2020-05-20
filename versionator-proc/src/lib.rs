use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

mod compiler;
mod version_control;

#[proc_macro]
pub fn versionator(id: TokenStream) -> TokenStream {
	let id = parse_macro_input!(id as Ident);

	let compiler_version = compiler::get_info();
	let version_control = version_control::get_info();

	let output = quote!(
		static #id: versionator::BuildInfo = versionator::BuildInfo{
			compiler: #compiler_version,
			version_control: #version_control,
		};
	);

	println!("{}", output.to_string());
	output.into()
}
