use proc_macro2::{Delimiter, Group, Ident, TokenStream};
use proc_macro_crate::crate_name;
use quote::{quote, TokenStreamExt};

use std::collections::VecDeque;

use versionator_common::{
	BuildInfo, CompilerChannel, CompilerVersion, CrateInfo, DateTime, GitInformation, Identifier, Utc, Version,
	VersionControl,
};

use crate::init_value::{init_value, InitValue};

pub(crate) fn indexed_init_value_tokens<T: IndexedInitValue>(this: &T, ids: VecDeque<String>) -> TokenStream {
	let mut tokens = TokenStream::new();
	this.indexed_init_value(ids, &mut tokens);
	tokens
}

pub(crate) fn indexed_init_value<T: IndexedInitValue>(this: &T, ids: VecDeque<String>, tokens: &mut TokenStream) {
	this.indexed_init_value(ids, tokens)
}

pub(crate) trait IndexedInitValue {
	fn indexed_init_value(&self, ids: VecDeque<String>, tokens: &mut TokenStream);
}

impl IndexedInitValue for BuildInfo {
	fn indexed_init_value(&self, mut ids: VecDeque<String>, tokens: &mut TokenStream) {
		if ids.is_empty() {
			return init_value(self, tokens);
		}

		let id = ids.pop_front().unwrap();
		match id.as_ref() {
			"timestamp" => indexed_init_value(&self.timestamp, ids, tokens),
			"crate_info" => indexed_init_value(&self.crate_info, ids, tokens),
			"compiler" => indexed_init_value(&self.compiler, ids, tokens),
			"version_control" => indexed_init_value(&self.version_control, ids, tokens),
			_ => panic!(format!("The member {} is not valid for versionator::BuildInfo", id)),
		}
	}
}

impl IndexedInitValue for CrateInfo {
	fn indexed_init_value(&self, mut ids: VecDeque<String>, tokens: &mut TokenStream) {
		if ids.is_empty() {
			return init_value(self, tokens);
		}

		let id = ids.pop_front().unwrap();
		match id.as_ref() {
			"name" => indexed_init_value(&self.name, ids, tokens),
			"version" => indexed_init_value(&self.version, ids, tokens),
			_ => panic!(format!("The member {} is not valid for versionator::CrateInfo", id)),
		}
	}
}

impl IndexedInitValue for DateTime<Utc> {
	fn indexed_init_value(&self, ids: VecDeque<String>, tokens: &mut TokenStream) {
		if ids.is_empty() {
			return init_value(self, tokens);
		}

		unimplemented!();
	}
}

impl IndexedInitValue for CompilerVersion {
	fn indexed_init_value(&self, mut ids: VecDeque<String>, tokens: &mut TokenStream) {
		if ids.is_empty() {
			return init_value(self, tokens);
		}

		let id = ids.pop_front().unwrap();
		match id.as_ref() {
			"version" => indexed_init_value(&self.version, ids, tokens),
			"commit_hash" => indexed_init_value(&self.commit_hash, ids, tokens),
			"commit_date" => indexed_init_value(&self.commit_date, ids, tokens),
			"channel" => indexed_init_value(&self.channel, ids, tokens),
			"host_triple" => indexed_init_value(&self.host_triple, ids, tokens),
			"target_triple" => indexed_init_value(&self.target_triple, ids, tokens),
			_ => panic!(format!(
				"The member {} is not valid for versionator::CompilerVersion",
				id
			)),
		}
	}
}

impl IndexedInitValue for Version {
	fn indexed_init_value(&self, mut ids: VecDeque<String>, tokens: &mut TokenStream) {
		if ids.is_empty() {
			return init_value(self, tokens);
		}

		let id = ids.pop_front().unwrap();
		match id.as_ref() {
			"major" => indexed_init_value(&self.major, ids, tokens),
			"minor" => indexed_init_value(&self.minor, ids, tokens),
			"patch" => indexed_init_value(&self.patch, ids, tokens),
			"pre" => indexed_init_value(&self.pre, ids, tokens),
			"build" => indexed_init_value(&self.build, ids, tokens),
			"to_string()" => indexed_init_value(&self.to_string(), ids, tokens),
			_ => panic!(format!("The member {} is not valid for versionator::Version", id)),
		}
	}
}

impl IndexedInitValue for Vec<Identifier> {
	fn indexed_init_value(&self, ids: VecDeque<String>, tokens: &mut TokenStream) {
		assert!(ids.is_empty());

		let versionator = Ident::new(
			&crate_name("versionator").expect("versionator must be a direct dependency"),
			proc_macro2::Span::call_site(),
		);

		let mut output = proc_macro2::TokenStream::new();
		output.append_all(quote!(&));
		let elements = proc_macro2::TokenStream::new();

		if !self.is_empty() {
			unimplemented!();
		}

		output.append(proc_macro2::Group::new(proc_macro2::Delimiter::Bracket, elements));
		output.append_all(quote!(as &[#versionator::Identifier]));
		tokens.append(Group::new(Delimiter::Parenthesis, output));
	}
}

impl IndexedInitValue for CompilerChannel {
	fn indexed_init_value(&self, mut ids: VecDeque<String>, tokens: &mut TokenStream) {
		let id = ids.pop_front();
		if let Some(id) = id {
			match id.as_ref() {
				"to_string()" => indexed_init_value(&self.to_string(), ids, tokens),
				_ => panic!(format!(
					"The member {} is not valid for versionator::CompilerChannel",
					id
				)),
			}
		} else {
			debug_assert!(ids.is_empty());

			let versionator = Ident::new(
				&crate_name("versionator").expect("versionator must be a direct dependency"),
				proc_macro2::Span::call_site(),
			);
			tokens.append_all(match self {
				versionator_common::CompilerChannel::Stable => quote!(#versionator::CompilerChannel::Stable),
				versionator_common::CompilerChannel::Beta => quote!(#versionator::CompilerChannel::Beta),
				versionator_common::CompilerChannel::Nightly => quote!(#versionator::CompilerChannel::Nightly),
				versionator_common::CompilerChannel::Dev => quote!(#versionator::CompilerChannel::Dev),
			});
		}
	}
}

impl<T: IndexedInitValue + InitValue> IndexedInitValue for Option<T> {
	fn indexed_init_value(&self, mut ids: VecDeque<String>, tokens: &mut TokenStream) {
		if ids.is_empty() {
			return init_value(self, tokens);
		}

		let id = ids.pop_front().unwrap();
		match id.as_ref() {
			"?" => match self {
				Some(value) => indexed_init_value(value, ids, tokens),
				None => panic!("Failed to unwrap element"),
			},
			_ => panic!(format!(
				"The member {} is not valid for Option<versionator::VersionControl>",
				id
			)),
		}
	}
}

impl IndexedInitValue for VersionControl {
	fn indexed_init_value(&self, ids: VecDeque<String>, tokens: &mut TokenStream) {
		match self {
			versionator_common::VersionControl::Git(value) => indexed_init_value(value, ids, tokens),
		}
	}
}

impl IndexedInitValue for GitInformation {
	fn indexed_init_value(&self, mut ids: VecDeque<String>, tokens: &mut TokenStream) {
		if ids.is_empty() {
			return init_value(self, tokens);
		}

		let id = ids.pop_front().unwrap();
		match id.as_ref() {
			"commit_hash" => indexed_init_value(&self.commit_hash, ids, tokens),
			"dirty" => indexed_init_value(&self.dirty, ids, tokens),
			"name" => indexed_init_value(&self.name, ids, tokens),
			_ => panic!(format!(
				"The member {} is not valid for versionator::GitInformation",
				id
			)),
		}
	}
}

impl IndexedInitValue for String {
	fn indexed_init_value(&self, ids: VecDeque<String>, tokens: &mut TokenStream) {
		assert!(ids.is_empty());

		tokens.append_all(quote!(#self));
	}
}

impl IndexedInitValue for u64 {
	fn indexed_init_value(&self, ids: VecDeque<String>, tokens: &mut TokenStream) {
		assert!(ids.is_empty());

		tokens.append_all(quote!(#self));
	}
}

impl IndexedInitValue for bool {
	fn indexed_init_value(&self, ids: VecDeque<String>, tokens: &mut TokenStream) {
		assert!(ids.is_empty());

		tokens.append_all(quote!(#self));
	}
}
