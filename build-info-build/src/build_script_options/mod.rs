use core::sync::atomic::{AtomicBool, Ordering};
use std::path::{Path, PathBuf};

use base64::write::EncoderWriter as Base64Encoder;
use build_info_common::{OptimizationLevel, VersionedString};
use chrono::{DateTime, Utc};

pub use self::crate_info::DependencyDepth;
use super::BuildInfo;

mod compiler;
mod crate_info;
mod target;
mod timestamp;
mod version_control;

pub fn cargo_toml() -> &'static Path {
	static CARGO_TOML: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
	CARGO_TOML.get_or_init(|| Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml"))
}

/// Type to store any (optional) options for the build script.
pub struct BuildScriptOptions {
	/// Stores if the build info has already been generated
	consumed: bool,

	/// Use this as the build timestamp, if set.
	timestamp: Option<DateTime<Utc>>,

	/// Enable dependency collection
	collect_dependencies: DependencyDepth,
}
static BUILD_SCRIPT_RAN: AtomicBool = AtomicBool::new(false);

impl BuildScriptOptions {
	/// WARNING: Should only be called once!
	fn drop_to_build_info(&mut self) -> BuildInfo {
		assert!(!self.consumed);
		self.consumed = true;

		let profile = std::env::var("PROFILE").unwrap_or_else(|_| "UNKNOWN".to_string());
		let optimization_level = match std::env::var("OPT_LEVEL")
			.expect("Expected environment variable `OPT_LEVEL` to be set by cargo")
			.as_str()
		{
			"0" => OptimizationLevel::O0,
			"1" => OptimizationLevel::O1,
			"2" => OptimizationLevel::O2,
			"3" => OptimizationLevel::O3,
			"s" => OptimizationLevel::Os,
			"z" => OptimizationLevel::Oz,
			level => panic!("Unknown optimization level {level:?}"),
		};

		let compiler = compiler::get_info();
		let target = target::get_info();
		let crate_info::Manifest {
			crate_info,
			workspace_root,
		} = crate_info::read_manifest(&target.triple, self.collect_dependencies);
		let version_control = version_control::get_info();

		let timestamp = self.timestamp.unwrap_or_else(timestamp::get_timestamp);
		let build_info = BuildInfo {
			timestamp,
			profile,
			optimization_level,
			crate_info,
			compiler,
			target,
			version_control,
		};

		let mut bytes = Vec::new();
		const BASE64_ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
			&base64::alphabet::STANDARD,
			base64::engine::GeneralPurposeConfig::new()
				.with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
		);
		let string_safe = Base64Encoder::new(&mut bytes, &BASE64_ENGINE);
		let mut compressed = zstd::Encoder::new(string_safe, 22).expect("Could not create ZSTD encoder");
		bincode::serialize_into(&mut compressed, &build_info).unwrap();
		compressed.finish().unwrap().finish().unwrap();

		let string = String::from_utf8(bytes).unwrap();
		let versioned = VersionedString::build_info_common_versioned(string);
		let serialized = serde_json::to_string(&versioned).unwrap();

		println!("cargo:rustc-env=BUILD_INFO={serialized}");

		// Whenever any `cargo:rerun-if-changed` key is set, the default set is cleared.
		// Since we will need to emit such keys to trigger rebuilds when the vcs repository changes state,
		// we also have to emit the customary triggers again, or we will only be rerun in that exact case.
		rebuild_if_project_changes(&workspace_root);

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
		assert!(!build_script_ran, "The build script may only be run once.");

		Self {
			consumed: false,
			timestamp: None,
			collect_dependencies: DependencyDepth::None,
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

/// Emits a `cargo:rerun-if-changed` line for each file in the target project.
/// By default, the following files are included:
/// - `Cargo.toml`
/// - `$workspace_root/Cargo.lock`
/// - Any file that ends in `.rs`
fn rebuild_if_project_changes(workspace_root: &str) {
	println!("cargo:rerun-if-changed={}", cargo_toml().to_str().unwrap());
	println!(
		"cargo:rerun-if-changed={}",
		Path::new(workspace_root).join("Cargo.lock").to_str().unwrap()
	);

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
		println!("cargo:rerun-if-changed={}", source.to_str().unwrap());
	}
}
