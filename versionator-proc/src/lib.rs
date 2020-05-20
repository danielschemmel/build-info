use proc_macro::TokenStream;
use quote::quote;
use syn::parse;
use syn::{parse_macro_input, Ident, Token, Visibility};

struct VersionatorSyntax {
	visibility: Option<Visibility>,
	id: Ident,
}

impl parse::Parse for VersionatorSyntax {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let visibility: Option<Visibility> = input.parse().ok();
		input.parse::<Token![fn]>()?;
		let id: Ident = input.parse()?;
		Ok(VersionatorSyntax { visibility, id })
	}
}

#[proc_macro]
pub fn versionator(input: TokenStream) -> TokenStream {
	let VersionatorSyntax { visibility, id } = parse_macro_input!(input as VersionatorSyntax);
	let visibility = visibility.map_or(quote!(), |vis| quote!(#vis));

	let buildinfo = versionator_common::BuildInfo::deserialize(&std::env::var("VERSIONATOR").unwrap());

	let output = quote! {
		#visibility fn #id() -> &'static versionator::BuildInfo {
			versionator::lazy_static! {
				static ref VERSION: versionator::BuildInfo = #buildinfo;
			}
			&VERSION
		}
	};

	println!("{}", output.to_string());
	output.into()
}

#[proc_macro]
pub fn versionator_str(input: TokenStream) -> TokenStream {
	let id = parse_macro_input!(input as Ident);
	let output = quote! {
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
