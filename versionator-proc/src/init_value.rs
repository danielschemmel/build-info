use proc_macro2::{Delimiter, Group, Ident, TokenStream};
use proc_macro_crate::crate_name;
use quote::{quote, TokenStreamExt};

use versionator_common::{BuildInfo, CompilerChannel, CompilerVersion, GitInformation, Version, VersionControl};

pub(crate) fn init_value_tokens<T: InitValue>(this: &T) -> TokenStream {
	let mut tokens = TokenStream::new();
	this.to_tokens(&mut tokens);
	tokens
}

pub(crate) fn init_value<T: InitValue>(this: &T, tokens: &mut TokenStream) {
	this.to_tokens(tokens)
}

pub(crate) trait InitValue {
	fn to_tokens(&self, tokens: &mut TokenStream);
}

impl InitValue for BuildInfo {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let versionator = Ident::new(
			&crate_name("versionator").expect("versionator must be a direct dependency"),
			proc_macro2::Span::call_site(),
		);

		tokens.append_all(quote!(#versionator::BuildInfo));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(compiler:));
		init_value(&self.compiler, &mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(version_control: ));
		init_value(&self.version_control, &mut initializer);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

impl InitValue for CompilerVersion {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let versionator = Ident::new(
			&crate_name("versionator").expect("versionator must be a direct dependency"),
			proc_macro2::Span::call_site(),
		);

		tokens.append_all(quote!(#versionator::CompilerVersion));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(version:));
		init_value(&self.version, &mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(commit_hash:));
		init_value(&self.commit_hash, &mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(commit_date:));
		init_value(&self.commit_date, &mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(channel:));
		init_value(&self.channel, &mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(host_triple:));
		init_value(&self.host_triple, &mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(target_triple:));
		init_value(&self.target_triple, &mut initializer);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

impl InitValue for CompilerChannel {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let versionator = Ident::new(
			&crate_name("versionator").expect("versionator must be a direct dependency"),
			proc_macro2::Span::call_site(),
		);

		match self {
			CompilerChannel::Dev => tokens.append_all(quote!(#versionator::CompilerChannel::Dev)),
			CompilerChannel::Nightly => tokens.append_all(quote!(#versionator::CompilerChannel::Nightly)),
			CompilerChannel::Beta => tokens.append_all(quote!(#versionator::CompilerChannel::Beta)),
			CompilerChannel::Stable => tokens.append_all(quote!(#versionator::CompilerChannel::Stable)),
		};
	}
}

impl<T: InitValue> InitValue for Option<T> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		if let Some(value) = self {
			tokens.append_all(quote!(Some));
			let mut initializer = TokenStream::new();
			init_value(value, &mut initializer);
			tokens.append(Group::new(Delimiter::Parenthesis, initializer));
		} else {
			tokens.append_all(quote!(None));
		}
	}
}

impl InitValue for String {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(#self.to_string()));
	}
}

impl InitValue for Version {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let versionator = Ident::new(
			&crate_name("versionator").expect("versionator must be a direct dependency"),
			proc_macro2::Span::call_site(),
		);

		let version_string = self.to_string();
		tokens.append_all(quote!(#versionator::Version::parse(#version_string).unwrap()));
	}
}

impl InitValue for VersionControl {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let versionator = Ident::new(
			&crate_name("versionator").expect("versionator must be a direct dependency"),
			proc_macro2::Span::call_site(),
		);

		match self {
			VersionControl::Git(data) => {
				tokens.append_all(quote!(#versionator::VersionControl::Git));
				let mut initializer = TokenStream::new();
				init_value(data, &mut initializer);
				tokens.append(Group::new(Delimiter::Parenthesis, initializer));
			}
		}
	}
}

impl InitValue for GitInformation {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let versionator = Ident::new(
			&crate_name("versionator").expect("versionator must be a direct dependency"),
			proc_macro2::Span::call_site(),
		);

		tokens.append_all(quote!(#versionator::GitInformation));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(commit_hash:));
		init_value(&self.commit_hash, &mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(dirty:));
		init_value(&self.dirty, &mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(name:));
		init_value(&self.name, &mut initializer);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

impl InitValue for bool {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(#self));
	}
}

impl InitValue for u64 {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(#self));
	}
}
