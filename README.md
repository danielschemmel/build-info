# Usage
Begin by adding `build-info` as a `[dependency]` and `build-info-build` as a `[build-dependency]` to your [`Cargo.toml`](sample/Cargo.toml).
By separating those two crates, pure compile-time dependencies, such as `git2` are not compiled into your final program.
For this to work properly, [ensure to opt in to resolver "2"](https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html#cargos-new-feature-resolver).
Please also make sure that both dependencies use the same version!

If it does not already exist, add a [`build.rs`](https://github.com/danielschemmel/build-info/tree/main/sample/build.rs) to your project's root, where you call `build_info_build::build_script()`.
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

The [sample](https://github.com/danielschemmel/build-info/tree/main/sample) project shows both variants.

## Features
The `build_info` package supports several feature flags:
- The `runtime` feature enables `build_info::build_info!`. It is enabled by default, but if you intend to only use `build_info::format!`, it is safe to disable this flag.
- The `nested` feature adds support for [`proc-macro-nested`](https://crates.io/crates/proc-macro-nested), which lets the `build_info::format!` macro be nested inside other proc-macros. This may require you to set `#![recursion_limit = "..."]` in your crate. The feature is disabled by default.
- The `chrono` feature enables the default features of the `chrono` package, which is used by `build_info::build_info!`. It is disabled by default.
- The `pyo3` feature enables the use of `build_info` types in a `pyo3`-Python enabled application, including extension modules. For example, a function `build_info::build_info!(fn version);` can be added to a module by using `module.add_function(wrap_pyfunction!(version, m)?)?;`. Note that this feature is *not* needed to just add the `__version__` tag to your module, which can be facilitated via `my_module.add("__version__", build_info::format!("{}", $.crate_info.version))?;`.
- The `serde` feature adds `Serialize`/`Deserialize` support to the types used by `build_info::build_info!`. It is disabled by default.

# Caveats
As of the time of writing, Rust does not support function-like proc-macros used as expressions.
The `format!` macro can often still be used as an expression, thanks to [the `proc-macro-hack` crate](https://crates.io/crates/proc-macro-hack).
However, its result will not behave like a string literal in all cases; for example, it cannot be used as an argument to `concat!`.

The build script will ask cargo to rerun it whenever the project or the currently checked out commit changes.
It will not necessarily be rerun if only the dependencies change (`build_info_build::build_script` will try to find the lockfile and depend on it, but it is not really aware of any of the more intricate features, such as, cargo workspaces).
Please open an issue if your specific use case requires a more strict rerun policy for `build.rs` and include a short description what additional files should trigger a rebuild when changed.
