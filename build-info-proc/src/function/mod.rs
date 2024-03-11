use build_info_common::BuildInfo;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse, parse_macro_input, Attribute, Ident, Token, Visibility};

struct FunctionSyntax {
	attrs: Vec<Attribute>,
	definition_crate: Ident,
	visibility: Option<Visibility>,
	id: Ident,
}

impl parse::Parse for FunctionSyntax {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let definition_crate = input.parse::<Ident>()?;

		let attrs = input.call(Attribute::parse_outer)?;
		let visibility: Option<Visibility> = input.parse().ok();
		input.parse::<Token![fn]>()?;
		let id = input.parse::<Ident>()?;

		Ok(FunctionSyntax {
			attrs,
			definition_crate,
			visibility,
			id,
		})
	}
}

pub fn build_info(input: TokenStream, build_info: BuildInfo) -> TokenStream {
	let FunctionSyntax {
		attrs,
		definition_crate,
		visibility,
		id,
	} = parse_macro_input!(input as FunctionSyntax);
	let visibility = visibility.map_or(quote!(), |vis| quote!(#vis));

	let bytes = bincode::serialize(&build_info).unwrap();
	let bytes = proc_macro2::Literal::byte_string(&bytes);

	#[allow(clippy::let_and_return)]
	let output = quote_spanned! {
		proc_macro::Span::mixed_site().into() =>
		#(#attrs)*
		#visibility fn #id() -> &'static #definition_crate::BuildInfo {
			static VERSION: ::std::sync::OnceLock<#definition_crate::BuildInfo> = ::std::sync::OnceLock::new();
			VERSION.get_or_init(|| #definition_crate::bincode::deserialize(#bytes).unwrap())
		}
	};

	// println!("{}", output.to_string());
	output.into()
}
