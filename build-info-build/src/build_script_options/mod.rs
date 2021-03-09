use base64::write::EncoderWriter as Base64Encoder;
use pretty_assertions::assert_eq;
use xz2::write::XzEncoder;

use core::sync::atomic::{AtomicBool, Ordering};
use std::path::Path;

use build_info_common::VersionedString;

use super::chrono::{DateTime, Utc};
use super::BuildInfo;

mod compiler;
mod crate_info;
mod timestamp;
mod version_control;

/// Type to store any (optional) options for the build script.
pub struct BuildScriptOptions {
	/// Stores if the build info has already been generated
	consumed: bool,

	/// Use this as the build timestamp, if set.
	timestamp: Option<DateTime<Utc>>,

	/// Enable dependency collection
	collect_dependencies: bool,
}
static BUILD_SCRIPT_RAN: AtomicBool = AtomicBool::new(false);

impl BuildScriptOptions {
	/// WARNING: Should only be called once!
	fn drop_to_build_info(&mut self) -> BuildInfo {
		assert_eq!(self.consumed, false);
		self.consumed = true;

		// Whenever any `cargo:rerun-if-changed` key is set, the default set is cleared.
		// Since we will need to emit such keys to trigger rebuilds when the vcs repository changes state,
		// we also have to emit the customary triggers again, or we will only be rerun in that exact case.
		rebuild_if_project_changes();

		let profile = std::env::var("PROFILE").unwrap_or_else(|_| "UNKNOWN".to_string());
		let optimization_level = std::env::var("OPT_LEVEL")
			.expect("Expected environment variable `OPT_LEVEL` to be set by cargo")
			.parse()
			.expect("Expected environment variable `OPT_LEVEL` to be set to a number by cargo");

		let compiler = compiler::get_info();
		let crate_info = crate_info::read_manifest(&compiler.target_triple, self.collect_dependencies);
		let version_control = version_control::get_info();

		let timestamp = self.timestamp.unwrap_or_else(timestamp::get_timestamp);
		let build_info = BuildInfo {
			timestamp,
			profile,
			optimization_level,
			crate_info,
			compiler,
			version_control,
		};

		let mut bytes = Vec::new();
		let string_safe = Base64Encoder::new(&mut bytes, base64::STANDARD_NO_PAD);
		let mut compressed = XzEncoder::new(string_safe, 9);
		bincode::serialize_into(&mut compressed, &build_info).unwrap();
		compressed.finish().unwrap().finish().unwrap();

		let string = String::from_utf8(bytes).unwrap();
		let versioned = VersionedString::build_info_common_versioned(string);
		let serialized = serde_json::to_string(&versioned).unwrap();

		println!("cargo:rustc-env=BUILD_INFO={}", serialized);

		build_info
	}

	/// Consumes the `BuildScriptOptions` and returns a `BuildInfo` object. Use this function if you wish to inspect the
	/// generated build information in `build.rs`.
	pub fn build(mut self) -> BuildInfo {
		self.drop_to_build_info()
	}
}

impl From<BuildScriptOptions> for BuildInfo {
	fn from(opts: BuildScriptOptions) -> BuildInfo {
		opts.build()
	}
}

impl Default for BuildScriptOptions {
	fn default() -> Self {
		let build_script_ran = BUILD_SCRIPT_RAN.swap(true, Ordering::Relaxed);
		assert_eq!(build_script_ran, false, "The build script may only be run once.");

		Self {
			consumed: false,
			timestamp: None,
			collect_dependencies: false,
		}
	}
}

impl Drop for BuildScriptOptions {
	fn drop(&mut self) {
		if !self.consumed {
			let _build_info = self.drop_to_build_info();
		}
	}
}

/// Emits a `cargo:rerun-if-changed` line for each file of the target project.
fn rebuild_if_project_changes() {
	println!("cargo:rerun-if-changed=Cargo.toml");
	if Path::new("Cargo.lock").is_file() {
		println!("cargo:rerun-if-changed=Cargo.lock");
	} else if Path::new("../Cargo.lock").is_file() {
		println!("cargo:rerun-if-changed=../Cargo.lock");
	}

	for source in glob::glob_with(
		"**/*.rs",
		glob::MatchOptions {
			case_sensitive: false,
			require_literal_separator: false,
			require_literal_leading_dot: false,
		},
	)
	.unwrap()
	.map(|source| source.unwrap())
	{
		println!("cargo:rerun-if-changed={}", source.to_string_lossy());
	}
}
