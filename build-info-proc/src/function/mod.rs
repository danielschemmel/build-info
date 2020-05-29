use proc_macro::TokenStream;
use proc_macro_crate::crate_name;
use quote::quote;
use syn::parse;
use syn::{parse_macro_input, Ident, Token, Visibility};

use build_info_common::BuildInfo;

mod init_value;
use init_value::init_value;

struct FunctionSyntax {
	visibility: Option<Visibility>,
	id: Ident,
}

impl parse::Parse for FunctionSyntax {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let visibility: Option<Visibility> = input.parse().ok();
		input.parse::<Token![fn]>()?;
		let id: Ident = input.parse()?;
		Ok(FunctionSyntax { visibility, id })
	}
}

pub fn build_info(input: TokenStream, build_info: BuildInfo) -> TokenStream {
	let build_info_crate = Ident::new(
		&crate_name("build-info").expect("build-info must be a direct dependency"),
		proc_macro2::Span::call_site(),
	);

	let FunctionSyntax { visibility, id } = parse_macro_input!(input as FunctionSyntax);
	let visibility = visibility.map_or(quote!(), |vis| quote!(#vis));

	let mut tokens = proc_macro2::TokenStream::new();
	init_value(&build_info, &mut tokens);

	#[allow(clippy::let_and_return)]
	let output = quote! {
		#visibility fn #id() -> &'static #build_info_crate::BuildInfo {
			#build_info_crate::lazy_static! {
				static ref VERSION: #build_info_crate::BuildInfo = #tokens;
			}
			&VERSION
		}
	};

	// println!("{}", output.to_string());
	output.into()
}
