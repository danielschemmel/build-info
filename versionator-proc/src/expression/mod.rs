use proc_macro::TokenStream;
use syn::parse;
use syn::{parse_macro_input, Ident, LitInt, Token};

use std::collections::VecDeque;

mod indexed_init_value;
use indexed_init_value::indexed_init_value_tokens;

pub fn version(input: TokenStream) -> TokenStream {
	let trace = parse_macro_input!(input as TraceSyntax);
	let buildinfo = versionator_common::BuildInfo::deserialize(&std::env::var("VERSIONATOR").unwrap());

	#[allow(clippy::let_and_return)]
	let output = indexed_init_value_tokens(&buildinfo, trace.ids);

	// println!("{}", output.to_string());
	output.into()
}

struct TraceSyntax {
	ids: VecDeque<String>,
}

impl parse::Parse for TraceSyntax {
	fn parse(input: parse::ParseStream) -> parse::Result<Self> {
		let mut trace = TraceSyntax { ids: VecDeque::new() };
		while !input.is_empty() {
			let lookahead = input.lookahead1();
			if lookahead.peek(Token![.]) {
				input.parse::<Token![.]>()?;

				let lookahead = input.lookahead1();
				if lookahead.peek(Ident) {
					let mut id = input.parse::<Ident>()?.to_string();
					if input.peek(syn::token::Paren) {
						let content;
						syn::parenthesized!(content in input);
						assert!(
							content.is_empty(),
							"Function calls with parameters are not currently supported"
						);
						id += "()";
					}
					trace.ids.push_back(id);
				} else if lookahead.peek(LitInt) {
					trace.ids.push_back(input.parse::<LitInt>()?.to_string());
				} else {
					return Err(lookahead.error());
				}
			} else if lookahead.peek(Token![?]) {
				input.parse::<Token![?]>()?;
				trace.ids.push_back("?".to_string());
			} else {
				return Err(lookahead.error());
			}
		}
		Ok(trace)
	}
}
