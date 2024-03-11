#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "runtime"), no_std)]

#[cfg(feature = "runtime")]
pub use build_info_common::{
	chrono, semver, BuildInfo, CompilerChannel, CompilerInfo, CpuInfo, CrateInfo, Endianness, GitInfo, OptimizationLevel,
	TargetInfo, VersionControl,
};
/// This crate defines macro_rules that pass `$crate` (i.e., this crate) to the proc-macros doing the actual work
/// The proc-macro crate that contains said proc-macros is reexported here, to be found in the macro_rules.
#[doc(hidden)]
pub use build_info_proc as proc;

/**
Generates a function that returns a reference to the `BuildInfo` structure for the crate.

Usage: `build_info!(fn build_info_function);`
*/
#[cfg(feature = "runtime")]
#[macro_export]
macro_rules! build_info {
	($($tokens:tt)*) => { $crate::proc::build_info!{$crate $($tokens)*} };
}

/// Used by the function generated by `build_info!` to deserialize the build information
#[cfg(feature = "runtime")]
#[doc(hidden)]
pub use bincode;
/**
Generates a string at compile-time that includes build information.

This function-like macro takes a single string-literal as its argument, on which it performs string interpolation with
the current build information. To do so, you can use a subset of the normal format language, with the special
"variable" `$` that denotes the `BuildInfo` object. For example, `build_info::format!("Built at {}", $.timestamp)`
might return "Built at 2020-05-28 20:09:40Z".`

You can use `?` to unwrap `Option`s and some additional types can be formatted this way (e.g., `Vec<T>`).

Literal curly braces can be printed by doubling them up: `build_info::format!("{{}}") // yields "{}"`.
*/
pub use build_info_proc::format;
