use proc_macro::TokenStream;
use proc_macro_crate::crate_name;
use quote::quote;
use syn::parse;
use syn::{parse_macro_input, Ident, Token, Visibility};

use crate::init_value::init_value;

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

pub fn versionator(input: TokenStream) -> TokenStream {
	let versionator = Ident::new(
		&crate_name("versionator").expect("versionator must be a direct dependency"),
		proc_macro2::Span::call_site(),
	);

	let VersionatorSyntax { visibility, id } = parse_macro_input!(input as VersionatorSyntax);
	let visibility = visibility.map_or(quote!(), |vis| quote!(#vis));

	let buildinfo = versionator_common::BuildInfo::deserialize(&std::env::var("VERSIONATOR").unwrap());
	let mut tokens = proc_macro2::TokenStream::new();
	init_value(&buildinfo, &mut tokens);

	#[allow(clippy::let_and_return)]
	let output = quote! {
		#visibility fn #id() -> &'static #versionator::BuildInfo {
			#versionator::lazy_static! {
				static ref VERSION: #versionator::BuildInfo = #tokens;
			}
			&VERSION
		}
	};

	// println!("{}", output.to_string());
	output.into()
}
