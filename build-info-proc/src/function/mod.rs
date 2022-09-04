use build_info_common::BuildInfo;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse, parse_macro_input, Ident, Token, Visibility};

mod init_value;
use init_value::init_value;

struct FunctionSyntax {
	definition_crate: Ident,
	visibility: Option<Visibility>,
	id: Ident,
}

impl parse::Parse for FunctionSyntax {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let definition_crate = input.parse::<Ident>()?;
		let visibility: Option<Visibility> = input.parse().ok();
		input.parse::<Token![fn]>()?;
		let id = input.parse::<Ident>()?;
		Ok(FunctionSyntax {
			definition_crate,
			visibility,
			id,
		})
	}
}

pub fn build_info(input: TokenStream, build_info: BuildInfo) -> TokenStream {
	let FunctionSyntax {
		definition_crate,
		visibility,
		id,
	} = parse_macro_input!(input as FunctionSyntax);
	let visibility = visibility.map_or(quote!(), |vis| quote!(#vis));

	let mut tokens = proc_macro2::TokenStream::new();
	init_value(&build_info, &mut tokens, &definition_crate);

	#[allow(clippy::let_and_return)]
	let output = quote_spanned! {
		proc_macro::Span::mixed_site().into() =>
		#visibility fn #id() -> &'static #definition_crate::BuildInfo {
			static VERSION: #definition_crate::Lazy<#definition_crate::BuildInfo> = #definition_crate::Lazy::new(|| #tokens);
			&VERSION
		}
	};

	// println!("{}", output.to_string());
	output.into()
}
