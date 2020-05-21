use proc_macro::TokenStream;
use proc_macro_crate::crate_name;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse;
use syn::{parse_macro_input, Ident, LitInt, Token, Visibility};

use std::collections::VecDeque;

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
	let versionator = Ident::new(&crate_name("versionator").expect("versionator must be a direct dependency"), proc_macro2::Span::call_site());

	let VersionatorSyntax { visibility, id } = parse_macro_input!(input as VersionatorSyntax);
	let visibility = visibility.map_or(quote!(), |vis| quote!(#vis));

	let buildinfo = versionator_common::BuildInfo::deserialize(&std::env::var("VERSIONATOR").unwrap());

	#[allow(clippy::let_and_return)]
	let output = quote! {
		#visibility fn #id() -> &'static #versionator::BuildInfo {
			#versionator::lazy_static! {
				static ref VERSION: #versionator::BuildInfo = #buildinfo;
			}
			&VERSION
		}
	};

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

#[proc_macro_hack]
pub fn version(input: TokenStream) -> TokenStream {
	let trace = parse_macro_input!(input as TraceSyntax);
	let buildinfo = versionator_common::BuildInfo::deserialize(&std::env::var("VERSIONATOR").unwrap());

	#[allow(clippy::let_and_return)]
	let output = buildinfo_value(trace.ids, buildinfo);

	// println!("{}", output.to_string());
	output
}

fn buildinfo_value(mut ids: VecDeque<String>, value: versionator_common::BuildInfo) -> TokenStream {
	if ids.is_empty() {
		return quote!(#value).into();
	}

	let id = ids.pop_front().unwrap();
	match id.as_ref() {
		"compiler" => compilerversion_value(ids, value.compiler),
		"version_control" => option_versioncontrol_value(ids, value.version_control),
		_ => panic!(format!("The member {} is not valid for versionator::BuildInfo", id)),
	}
}

fn compilerversion_value(mut ids: VecDeque<String>, value: versionator_common::CompilerVersion) -> TokenStream {
	if ids.is_empty() {
		return quote!(#value).into();
	}

	let id = ids.pop_front().unwrap();
	match id.as_ref() {
		"version" => version_value(ids, value.version),
		"commit_hash" => option_string_value(ids, value.commit_hash),
		"commit_date" => option_string_value(ids, value.commit_date),
		"channel" => compilerchannel_value(ids, value.channel),
		"host_triple" => raw_value(ids, &value.host_triple),
		"target_triple" => raw_value(ids, &value.target_triple),
		_ => panic!(format!(
			"The member {} is not valid for versionator::CompilerVersion",
			id
		)),
	}
}

fn version_value(mut ids: VecDeque<String>, value: versionator_common::Version) -> TokenStream {
	if ids.is_empty() {
		let version_string = value.to_string();
		return quote!(versionator::Version::parse(#version_string).unwrap()).into();
	}

	let id = ids.pop_front().unwrap();
	match id.as_ref() {
		"major" => raw_value(ids, &value.major),
		"minor" => raw_value(ids, &value.minor),
		"patch" => raw_value(ids, &value.patch),
		"pre" => vec_identifier_value(ids, value.pre),
		"build" => vec_identifier_value(ids, value.build),
		"to_string()" => raw_value(ids, &value.to_string()),
		_ => panic!(format!("The member {} is not valid for versionator::Version", id)),
	}
}

fn vec_identifier_value(ids: VecDeque<String>, value: Vec<versionator_common::Identifier>) -> TokenStream {
	assert!(ids.is_empty());

	use quote::TokenStreamExt;

	let mut output = proc_macro2::TokenStream::new();
	output.append_all(quote!(&));
	let elements = proc_macro2::TokenStream::new();

	if !value.is_empty() {
		unimplemented!();
	}

	output.append(proc_macro2::Group::new(proc_macro2::Delimiter::Bracket, elements));
	output.append_all(quote!(as &[versionator::Identifier]));
	grouped(output, proc_macro2::Delimiter::Parenthesis).into()
}

fn compilerchannel_value(mut ids: VecDeque<String>, value: versionator_common::CompilerChannel) -> TokenStream {
	let id = ids.pop_front();
	if let Some(id) = id {
		match id.as_ref() {
			"to_string()" => raw_value(ids, &value.to_string()),
			_ => panic!(format!(
				"The member {} is not valid for versionator::CompilerChannel",
				id
			)),
		}
	} else {
		debug_assert!(ids.is_empty());

		match value {
			versionator_common::CompilerChannel::Stable => quote!(versionator::CompilerChannel::Stable).into(),
			versionator_common::CompilerChannel::Beta => quote!(versionator::CompilerChannel::Beta).into(),
			versionator_common::CompilerChannel::Nightly => quote!(versionator::CompilerChannel::Nightly).into(),
			versionator_common::CompilerChannel::Dev => quote!(versionator::CompilerChannel::Dev).into(),
		}
	}
}

fn option_versioncontrol_value(
	mut ids: VecDeque<String>,
	value: Option<versionator_common::VersionControl>,
) -> TokenStream {
	if ids.is_empty() {
		return quote!(#value).into();
	}

	let id = ids.pop_front().unwrap();
	match id.as_ref() {
		"?" => versioncontrol_value(ids, value.unwrap()),
		_ => panic!(format!(
			"The member {} is not valid for Option<versionator::VersionControl>",
			id
		)),
	}
}

fn versioncontrol_value(ids: VecDeque<String>, value: versionator_common::VersionControl) -> TokenStream {
	match value {
		versionator_common::VersionControl::Git(value) => gitinformation_value(ids, value),
	}
}

fn gitinformation_value(mut ids: VecDeque<String>, value: versionator_common::GitInformation) -> TokenStream {
	if ids.is_empty() {
		return quote!(#value).into();
	}

	let id = ids.pop_front().unwrap();
	match id.as_ref() {
		"commit_hash" => raw_value(ids, &value.commit_hash),
		"dirty" => raw_value(ids, &value.dirty),
		"name" => option_string_value(ids, value.name),
		_ => panic!(format!(
			"The member {} is not valid for versionator::GitInformation",
			id
		)),
	}
}

fn option_string_value(mut ids: VecDeque<String>, value: Option<String>) -> TokenStream {
	if ids.is_empty() {
		return quote!(#value).into();
	}

	let id = ids.pop_front().unwrap();
	match id.as_ref() {
		"?" => raw_value(ids, &value.unwrap()),
		_ => panic!(format!(
			"The member {} is not valid for Option<versionator::VersionControl>",
			id
		)),
	}
}

fn raw_value<T: quote::ToTokens>(ids: VecDeque<String>, value: &T) -> TokenStream {
	assert!(ids.is_empty());
	quote!(#value).into()
}

fn grouped(tokens: proc_macro2::TokenStream, delimiter: proc_macro2::Delimiter) -> proc_macro2::TokenStream {
	use quote::TokenStreamExt;

	let mut output = proc_macro2::TokenStream::new();
	output.append(proc_macro2::Group::new(delimiter, tokens));
	output
}
