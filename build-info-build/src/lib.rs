use build_info_common::BuildInfo;

use std::path::Path;

mod compiler;
mod crate_info;
mod version_control;

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

/// Call this function in your `build.rs` script to generate the data consumed by the `build_info` crate.
pub fn build_script() {
	// Whenever any `cargo:rerun-if-changed` key is set, the default set is cleared.
	// Since we will need to emit such keys to trigger rebuilds when the vcs repository changes state,
	// we also have to emit the customary triggers again, or we will only be rerun in that exact case.
	rebuild_if_project_changes();

	let crate_info = crate_info::read_manifest();
	let compiler = compiler::get_info();
	let version_control = version_control::get_info();

	let timestamp = build_info_common::Utc::now();
	let build_info = BuildInfo {
		timestamp,
		crate_info,
		compiler,
		version_control,
	};

	println!(
		"cargo:rustc-env=VERSIONATOR={}",
		serde_json::to_string(&build_info).unwrap()
	);
}
