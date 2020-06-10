/*!
This crate is used to collect build info for consumption by the `build-info` crate.

```rust,no_run
fn main() {
	// Calling `build_info_build::build_script` collects all data and makes it available to `build_info::build_info!`
	// and `build_info::format!` in the main program.
	build_info_build::build_script();
}
```

# Features
The ´build-info-build` crate has the following features:

- `git` (enabled by default): Enables git support. A git repository will only be detected if this feature is available.
*/

#![forbid(unsafe_code)]

pub use build_info_common::{
	chrono, semver, BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, GitInfo, VersionControl,
};

mod build_script_options;
pub use build_script_options::BuildScriptOptions;

/// Call this function in your `build.rs` script to generate the data consumed by the `build_info` crate.
/// Additional customization options are available by manipulating the return type.
/// The actual work is performed once the return type is dropped.
pub fn build_script() -> BuildScriptOptions {
	BuildScriptOptions::default()
}
