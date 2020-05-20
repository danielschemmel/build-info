use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn versionator(input: TokenStream) -> TokenStream {
	let id = parse_macro_input!(input as Ident);
	let output = quote!{
		versionator::lazy_static! {
			static ref #id: versionator::BuildInfo = versionator::BuildInfo::deserialize(env!("VERSIONATOR"));
		}
	};

	println!("{:?}", output.to_string());
	output.into()
}
