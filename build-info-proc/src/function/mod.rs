use proc_macro::TokenStream;
use proc_macro_crate::crate_name;
use quote::quote;
use syn::parse;
use syn::{parse_macro_input, Ident, Token, Visibility};

use build_info_common::BuildInfo;

mod init_value;
use init_value::init_value;

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

pub fn build_info(input: TokenStream) -> TokenStream {
	let build_info = Ident::new(
		&crate_name("build-info").expect("build-info must be a direct dependency"),
		proc_macro2::Span::call_site(),
	);

	let VersionatorSyntax { visibility, id } = parse_macro_input!(input as VersionatorSyntax);
	let visibility = visibility.map_or(quote!(), |vis| quote!(#vis));

	let buildinfo: BuildInfo = serde_json::from_str(&std::env::var("VERSIONATOR").unwrap()).unwrap();
	let mut tokens = proc_macro2::TokenStream::new();
	init_value(&buildinfo, &mut tokens);

	#[allow(clippy::let_and_return)]
	let output = quote! {
		#visibility fn #id() -> &'static #build_info::BuildInfo {
			#build_info::lazy_static! {
				static ref VERSION: #build_info::BuildInfo = #tokens;
			}
			&VERSION
		}
	};

	// println!("{}", output.to_string());
	output.into()
}
