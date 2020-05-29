/*!
Begin by adding `build-info` as a `[dependency]` and `build-info-build` as a `[build-dependency]` to your [`Cargo.toml`](https://github.com/danielschemmel/build-info/tree/master/sample/Cargo.toml).
Please make sure that both dependencies use the same version!

If it does not already exist, add a [`build.rs`](https://github.com/danielschemmel/build-info/tree/master/sample/build.rs) to your project's root, where you call `build_info_build::build_script()`.
This will collect build information at compile time.

Then, either use the `build_info!` macro to add a function that returns version information at runtime:
```rust
build_info::build_info!(fn version);
```
or use `build_info::format!` to generate a string at compile time:
```rust
// sample output: "{sample v0.0.6 built with rustc version 1.43.1 8d69840ab92ea7f4d323420088dd8c9775f180cd at 2020-05-28 20:09:40.379213639Z}"
build_info::format!("{{{.crate_info.name} v{.crate_info.version} built with rustc version {.compiler.version} {.compiler.commit_id} at {.timestamp}}}")
```

You can also check out the [sample](https://github.com/danielschemmel/build-info/tree/master/sample/) project that shows both variants.

# Features
The ´build-info-build` crate has the following features:

- `git` (enabled by default): Enables git support. A git repository will only be detected if this feature is available.
*/

#![forbid(unsafe_code)]

use build_info_common::{BuildInfo, VersionedString};

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

	let versioned = VersionedString::build_info_common_versioned(serde_json::to_string(&build_info).unwrap());

	println!(
		"cargo:rustc-env=BUILD_INFO={}",
		serde_json::to_string(&versioned).unwrap()
	);
}
