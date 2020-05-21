use proc_macro2::{Delimiter, Group, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

use crate::{Version, VersionControl};

pub(crate) trait ToTokensExt {
	fn to_tokens_ext(&self, tokens: &mut TokenStream);
}

impl<T: ToTokensExt> ToTokensExt for Option<T> {
	fn to_tokens_ext(&self, tokens: &mut TokenStream) {
		if let Some(value) = self {
			tokens.append_all(quote!(Some));
			let mut initializer = TokenStream::new();
			value.to_tokens_ext(&mut initializer);
			tokens.append(Group::new(Delimiter::Parenthesis, initializer));
		} else {
			tokens.append_all(quote!(None));
		}
	}
}

impl ToTokensExt for String {
	fn to_tokens_ext(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(#self.to_string()));
	}
}

impl ToTokensExt for Version {
	fn to_tokens_ext(&self, tokens: &mut TokenStream) {
		let version_string = self.to_string();
		tokens.append_all(quote!(versionator::Version::parse(#version_string).unwrap()));
	}
}

impl ToTokensExt for VersionControl {
	fn to_tokens_ext(&self, tokens: &mut TokenStream) {
		self.to_tokens(tokens);
	}
}
