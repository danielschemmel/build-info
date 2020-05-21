use derive_more::Display;
pub use semver::{Identifier, Version};
use serde::{Deserialize, Serialize};

#[cfg(feature = "proc-macro")]
use proc_macro2::{Delimiter, Group, TokenStream};
#[cfg(feature = "proc-macro")]
use quote::{quote, ToTokens, TokenStreamExt};

#[cfg(feature = "proc-macro")]
mod to_tokens_ext;

#[cfg(feature = "proc-macro")]
use crate::to_tokens_ext::ToTokensExt;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BuildInfo {
	pub compiler: CompilerVersion,
	pub version_control: Option<VersionControl>,
}

impl BuildInfo {
	pub fn serialize(&self) -> String {
		serde_json::to_string(self).unwrap()
	}

	pub fn deserialize(value: &str) -> Self {
		serde_json::from_str(value).unwrap()
	}
}

#[cfg(feature = "proc-macro")]
impl ToTokens for BuildInfo {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(versionator::BuildInfo));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(compiler:));
		self.compiler.to_tokens(&mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(version_control: ));
		self.version_control.to_tokens_ext(&mut initializer);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompilerVersion {
	pub version: Version,
	pub commit_hash: Option<String>,
	pub commit_date: Option<String>,
	pub channel: CompilerChannel,
	pub host_triple: String,
	pub target_triple: String,
}

#[cfg(feature = "proc-macro")]
impl ToTokens for CompilerVersion {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(versionator::CompilerVersion));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(version:));
		self.version.to_tokens_ext(&mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(commit_hash:));
		self.commit_hash.to_tokens_ext(&mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(commit_date:));
		self.commit_date.to_tokens_ext(&mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(channel:));
		self.channel.to_tokens(&mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(host_triple:));
		self.host_triple.to_tokens_ext(&mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(target_triple:));
		self.target_triple.to_tokens_ext(&mut initializer);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

#[derive(Display)]
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}

#[cfg(feature = "proc-macro")]
impl ToTokens for CompilerChannel {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			CompilerChannel::Dev => tokens.append_all(quote!(versionator::CompilerChannel::Dev)),
			CompilerChannel::Nightly => tokens.append_all(quote!(versionator::CompilerChannel::Nightly)),
			CompilerChannel::Beta => tokens.append_all(quote!(versionator::CompilerChannel::Beta)),
			CompilerChannel::Stable => tokens.append_all(quote!(versionator::CompilerChannel::Stable)),
		};
	}
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionControl {
	Git(GitInformation),
}

#[cfg(feature = "proc-macro")]
impl ToTokens for VersionControl {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			VersionControl::Git(data) => {
				tokens.append_all(quote!(versionator::VersionControl::Git));
				let mut initializer = TokenStream::new();
				data.to_tokens(&mut initializer);
				tokens.append(Group::new(Delimiter::Parenthesis, initializer));
			}
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitInformation {
	pub commit_hash: String,
	pub dirty: bool,
	pub name: Option<String>,
}

#[cfg(feature = "proc-macro")]
impl ToTokens for GitInformation {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(versionator::GitInformation));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(commit_hash:));
		self.commit_hash.to_tokens_ext(&mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(dirty:));
		self.dirty.to_tokens(&mut initializer);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(name:));
		self.name.to_tokens_ext(&mut initializer);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}
