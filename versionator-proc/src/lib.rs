use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

fn compiler_version() -> proc_macro2::TokenStream {
	use rustc_version::{version_meta, Channel};

	let version = version_meta().unwrap();

	let semver = version.semver.to_string();

	let commit_hash = if let Some(commit_hash) = version.commit_hash {
		quote!(Some(#commit_hash))
	} else {
		quote!(None)
	};

	let commit_date = if let Some(commit_date) = version.commit_date {
		quote!(Some(#commit_date))
	} else {
		quote!(None)
	};

	let channel = match version.channel {
		Channel::Stable => quote!(versionator::CompilerChannel::Stable),
		Channel::Beta => quote!(versionator::CompilerChannel::Beta),
		Channel::Nightly => quote!(versionator::CompilerChannel::Nightly),
		Channel::Dev => quote!(versionator::CompilerChannel::Dev),
	};

	let host_triple = &version.host;
	let target_triple = env!("TARGET_TRIPLE");

	quote!(versionator::CompilerVersion{
		version_str: #semver,
		commit_hash: #commit_hash,
		commit_date: #commit_date,
		channel: #channel,
		host_triple: #host_triple,
		target_triple: #target_triple,
	})
}

#[proc_macro]
pub fn versionator(id: TokenStream) -> TokenStream {
	let id = parse_macro_input!(id as Ident);
	let compiler_version = compiler_version();

	let output = quote!(
		static #id: versionator::BuildInfo = versionator::BuildInfo{
			compiler: #compiler_version
		};
	);

	println!("{}", output.to_string());
	output.into()
}
