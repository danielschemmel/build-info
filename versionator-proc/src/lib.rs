use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn versionator(input: TokenStream) -> TokenStream {
	let id = parse_macro_input!(input as Ident);
	let output = quote!{
		fn #id() -> &'static versionator::BuildInfo {
			versionator::lazy_static! {
				static ref VERSION: versionator::BuildInfo = versionator::BuildInfo::deserialize(env!("VERSIONATOR"));
			}
			&VERSION
		}
	};

	// println!("{:?}", output.to_string());
	output.into()
}
