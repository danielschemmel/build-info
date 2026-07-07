#![forbid(unsafe_code)]

use std::io::Cursor;

use build_info_common::{BuildInfo, VersionedString};
use manyhow::Emitter;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod format;
#[cfg(feature = "runtime")]
mod function;

/**
Call as `build_info!(fn name)` to create a function called `name` that returns a reference to a lazily created
and cached `BuildInfo` object.

This macro also accepts a visibility specifier for the generated function, such as `build_info!(pub fn version)`.
*/
#[cfg(feature = "runtime")]
#[manyhow::manyhow]
#[proc_macro]
pub fn build_info(input: TokenStream) -> manyhow::Result<TokenStream2> {
	function::build_info(input, deserialize_build_info()?)
}

#[manyhow::manyhow]
#[proc_macro]
pub fn format(input: TokenStream, emitter: &mut Emitter) -> manyhow::Result<TokenStream2> {
	let result = format::format(input, deserialize_build_info()?, emitter)?;
	emitter.into_result()?;
	Ok(result)
}

fn deserialize_build_info() -> manyhow::Result<BuildInfo> {
	let data = std::env::var("BUILD_INFO").map_err(|err| {
		manyhow::error_message!("No BuildInfo data found!";
			note = "Did you call build_info_build::build_script() in your build.rs?";
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
			note = "Caused by: {}", err
		)
	})?;

	// println!("Serialized data is {} bytes long.", data.len());

	let versioned: VersionedString = serde_json::from_str(&data).map_err(|err| {
		manyhow::error_message!("Could not deserialize BuildInfo data at all!";
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
			note = "Caused by: {}", err;
		)
	})?;

	if !versioned.check() {
		// TODO: This should really be a warning - but warnings are currently nightly-only...
		manyhow::bail!("BuildInfo data has an different version!";
			note = "The serialized data has version {}", versioned.version;
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
		);
	}

	let bytes = z85::decode(versioned.string.as_bytes()).map_err(|err| {
		manyhow::error_message!("BuildInfo data cannot be deserialized!";
			note = "The serialized data has version {}", versioned.version;
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
			note = "Underlying cause: {}", err;
		)
	})?;
	let cursor = Cursor::new(&bytes);
	let mut decoder = zstd::Decoder::new(cursor).expect("Could not crate ZSTD decoder");
	ciborium::from_reader(&mut decoder).map_err(|err| {
		manyhow::error_message!("BuildInfo data cannot be deserialized!";
			note = "The serialized data has version {}", versioned.version;
			note = "This crate expects version {} of the BuildInfo data", build_info_common::crate_version();
			note = "Underlying cause: {}", err;
		)
		.into()
	})
}
