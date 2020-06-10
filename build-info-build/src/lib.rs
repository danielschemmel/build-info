/*!
Begin by adding `build-info` as a `[dependency]` and `build-info-build` as a `[build-dependency]` to your [`Cargo.toml`](https://github.com/danielschemmel/build-info/tree/master/sample/Cargo.toml).
Please make sure that both dependencies use the same version!

If it does not already exist, add a [`build.rs`](https://github.com/danielschemmel/build-info/tree/master/sample/build.rs) to your project's root, where you call `build_info_build::build_script()`.
This will collect build information at compile time.

Then, either use the `build_info!` macro to add a function that returns version information at runtime:
```rust,ignore
build_info::build_info!(fn version);
```
or use `build_info::format!` to generate a string at compile time:
```rust,ignore
// sample output: "{sample v0.0.13 built with rustc 1.45.0-nightly (4bd32c980 2020-05-29) at 2020-05-30 11:22:46Z}"
build_info::format!("{{{} v{} built with {} at {}}}", $.crate_info.name, $.crate_info.version, $.compiler, $.timestamp)
```

You can also check out the [sample](https://github.com/danielschemmel/build-info/tree/master/sample/) project that shows both variants.

# Features
The ´build-info-build` crate has the following features:

- `dependencies` (disabled by default): (Recursively) collects all dependencies inside `$.crate_info.dependencies`.
	Enabling this feature may cause the serialized build data to become large enough to cause problems for cargo. Try it!
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
