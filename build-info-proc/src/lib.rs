#![forbid(unsafe_code)]

use base64::read::DecoderReader as Base64Decoder;
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, emit_call_site_error, proc_macro_error};
use proc_macro_hack::proc_macro_hack;
use xz2::read::XzDecoder;

use std::io::Cursor;

use build_info_common::{BuildInfo, VersionedString};

mod format;
#[cfg(feature = "runtime")]
mod function;

/**
Call as `build_info!(fn name)` to create a function called `name` that returns a reference to a lazily created
and cached `BuildInfo` object.

This macro also accepts a visibility specifier for the generated function, such as `build_info!(pub fn version)`.
*/
#[cfg(feature = "runtime")]
#[proc_macro_error]
#[proc_macro]
pub fn build_info(input: TokenStream) -> TokenStream {
	function::build_info(input, deserialize_build_info())
}

#[proc_macro_error]
#[proc_macro_hack]
pub fn format(input: TokenStream) -> TokenStream {
	format::format(input, deserialize_build_info())
}

fn deserialize_build_info() -> BuildInfo {
	let data = std::env::var("BUILD_INFO").unwrap_or_else(|err| {
		abort_call_site!("No BuildInfo data found!";
			note = "Did you call build_info_build::build_script() in your build.rs?";
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
			note = "Caused by: {}", err;
		)
	});

	// println!("Serialized data is {} bytes long.", data.len());

	let versioned: VersionedString = serde_json::from_str(&data).unwrap_or_else(|err| {
		abort_call_site!("Could not deserialize BuildInfo data at all!";
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
			note = "Caused by: {}", err;
		)
	});

	if !versioned.check() {
		// TODO: This should really be a warning - but warnings are currently nightly-only...
		emit_call_site_error!("BuildInfo data has an different version!";
			note = "The serialized data has version {}", versioned.version;
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
		);
	}

	let mut cursor = Cursor::new(versioned.string.as_bytes());
	let string_safe = Base64Decoder::new(&mut cursor, base64::STANDARD_NO_PAD);
	let decoder = XzDecoder::new(string_safe);
	bincode::deserialize_from(decoder).unwrap_or_else(|err| {
		abort_call_site!("BuildInfo data cannot be deserialized!";
			note = "The serialized data has version {}", versioned.version;
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
			note = "Underlying cause: {}", err;
		)
	})
}
