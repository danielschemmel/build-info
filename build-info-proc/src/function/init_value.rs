use build_info_common::{
	chrono::{DateTime, Datelike, NaiveDate, Utc},
	semver::Version,
	BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, GitInfo, OptimizationLevel, VersionControl,
};
use proc_macro2::{Delimiter, Group, Ident, TokenStream};
use quote::{quote, quote_spanned, TokenStreamExt};

pub(crate) fn init_value<T: InitValue>(this: &T, tokens: &mut TokenStream, definition_crate: &Ident) {
	this.init_value(tokens, definition_crate)
}

pub(crate) trait InitValue {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident);
}

impl InitValue for BuildInfo {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		tokens.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::BuildInfo));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(timestamp:));
		init_value(&self.timestamp, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(profile:));
		init_value(&self.profile, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(optimization_level:));
		init_value(&self.optimization_level, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(crate_info:));
		init_value(&self.crate_info, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(compiler:));
		init_value(&self.compiler, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(version_control: ));
		init_value(&self.version_control, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

impl InitValue for OptimizationLevel {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		match self {
			OptimizationLevel::O0 => tokens
				.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::OptimizationLevel::O0)),
			OptimizationLevel::O1 => tokens
				.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::OptimizationLevel::O1)),
			OptimizationLevel::O2 => tokens
				.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::OptimizationLevel::O2)),
			OptimizationLevel::O3 => tokens
				.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::OptimizationLevel::O3)),
			OptimizationLevel::Os => tokens
				.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::OptimizationLevel::Os)),
			OptimizationLevel::Oz => tokens
				.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::OptimizationLevel::Oz)),
		};
	}
}

impl InitValue for CrateInfo {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		tokens.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::CrateInfo));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(name:));
		init_value(&self.name, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(version:));
		init_value(&self.version, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(authors:));
		init_value(&self.authors, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(license:));
		init_value(&self.license, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(enabled_features:));
		init_value(&self.enabled_features, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(available_features:));
		init_value(&self.available_features, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(dependencies:));
		init_value(&self.dependencies, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

impl InitValue for CompilerInfo {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		tokens.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::CompilerInfo));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(version:));
		init_value(&self.version, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(commit_id:));
		init_value(&self.commit_id, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(commit_date:));
		init_value(&self.commit_date, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(channel:));
		init_value(&self.channel, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(host_triple:));
		init_value(&self.host_triple, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(target_triple:));
		init_value(&self.target_triple, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

impl InitValue for CompilerChannel {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		match self {
			CompilerChannel::Dev => tokens
				.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::CompilerChannel::Dev)),
			CompilerChannel::Nightly => tokens.append_all(
				quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::CompilerChannel::Nightly),
			),
			CompilerChannel::Beta => tokens
				.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::CompilerChannel::Beta)),
			CompilerChannel::Stable => tokens.append_all(
				quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::CompilerChannel::Stable),
			),
		};
	}
}

impl<T: InitValue> InitValue for Option<T> {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		if let Some(value) = self {
			tokens.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => Some));
			let mut initializer = TokenStream::new();
			init_value(value, &mut initializer, definition_crate);
			tokens.append(Group::new(Delimiter::Parenthesis, initializer));
		} else {
			tokens.append_all(quote!(None));
		}
	}
}

impl<T: InitValue> InitValue for Vec<T> {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		tokens.append_all(quote!(vec!));
		let mut initializer = TokenStream::new();

		let mut first = true;
		for element in self {
			if first {
				first = false;
			} else {
				initializer.append_all(quote!(,));
			}
			init_value(element, &mut initializer, definition_crate);
		}

		tokens.append(Group::new(Delimiter::Bracket, initializer));
	}
}

impl InitValue for VersionControl {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		match self {
			VersionControl::Git(data) => {
				tokens
					.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::VersionControl::Git));
				let mut initializer = TokenStream::new();
				init_value(data, &mut initializer, definition_crate);
				tokens.append(Group::new(Delimiter::Parenthesis, initializer));
			}
		}
	}
}

impl InitValue for GitInfo {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		tokens.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() => #definition_crate::GitInfo));
		let mut initializer = TokenStream::new();

		initializer.append_all(quote!(commit_id:));
		init_value(&self.commit_id, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(commit_short_id:));
		init_value(&self.commit_short_id, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(commit_timestamp:));
		init_value(&self.commit_timestamp, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(dirty:));
		init_value(&self.dirty, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(branch:));
		init_value(&self.branch, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		initializer.append_all(quote!(tags:));
		init_value(&self.tags, &mut initializer, definition_crate);
		initializer.append_all(quote!(,));

		tokens.append(Group::new(Delimiter::Brace, initializer));
	}
}

impl InitValue for Version {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		let version_string = self.to_string();
		tokens.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() =>
			#definition_crate::semver::Version::parse(#version_string).unwrap()
		));
	}
}

impl InitValue for DateTime<Utc> {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		let nanos = self.timestamp_nanos();
		tokens.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() =>
			#definition_crate::chrono::TimeZone::timestamp_nanos(&#definition_crate::chrono::Utc, #nanos)
		));
	}
}

impl InitValue for NaiveDate {
	fn init_value(&self, tokens: &mut TokenStream, definition_crate: &Ident) {
		let year = self.year();
		let month = self.month();
		let day = self.day();
		tokens.append_all(quote_spanned!(proc_macro::Span::mixed_site().into() =>
			#definition_crate::chrono::NaiveDate::from_ymd_opt(#year, #month, #day).unwrap()
		));
	}
}

impl InitValue for bool {
	fn init_value(&self, tokens: &mut TokenStream, _definition_crate: &Ident) {
		tokens.append_all(quote!(#self));
	}
}

impl InitValue for u8 {
	fn init_value(&self, tokens: &mut TokenStream, _definition_crate: &Ident) {
		tokens.append_all(quote!(#self));
	}
}

impl InitValue for u64 {
	fn init_value(&self, tokens: &mut TokenStream, _definition_crate: &Ident) {
		tokens.append_all(quote!(#self));
	}
}

impl InitValue for String {
	fn init_value(&self, tokens: &mut TokenStream, _definition_crate: &Ident) {
		tokens.append_all(quote!(#self.to_string()));
	}
}

impl InitValue for str {
	fn init_value(&self, tokens: &mut TokenStream, _definition_crate: &Ident) {
		tokens.append_all(quote!(#self));
	}
}
