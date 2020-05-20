use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

fn compiler_version() -> proc_macro2::TokenStream {
	use rustc_version::{version_meta, Channel};

	let version = version_meta().unwrap();

	let channel = match version.channel {
		Channel::Stable => quote!(versionator::CompilerChannel::Stable),
		Channel::Beta => quote!(versionator::CompilerChannel::Beta),
		Channel::Nightly => quote!(versionator::CompilerChannel::Nightly),
		Channel::Dev => quote!(versionator::CompilerChannel::Dev),
	};

	let host_triple = &version.host;
	let target_triple = env!("TARGET_TRIPLE");

	quote!(versionator::CompilerVersion{
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
		static #id: versionator::Version = versionator::Version{
			compiler: #compiler_version
		};
	);

	// println!("{:?}", output.to_string());
	output.into()
}
