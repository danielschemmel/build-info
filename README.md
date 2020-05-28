# Usage
Begin by adding `build-info` as a `[dependency]` and `build-info-build` as a `[build-dependency]` to your [`Cargo.toml`](sample/Cargo.toml).
By separating those two crates, pure compile-time dependencies, such as `git2` are not compiled into your final program.

If it does not already exist, add a [`build.rs`](sample/build.rs) to your project's root, where you call `build_info_build::build_script()`.
This will collect build information at compile time.

Then, either use the `build_info!` macro to add a function that returns version information at runtime:
```rust
build_info::build_info!(fn version);
```
or use `build_info::format!` to generate a string at compile time:
```rust
// sample output: "{sample v0.0.6 built with rustc version 1.43.1 8d69840ab92ea7f4d323420088dd8c9775f180cd at 2020-05-28 20:09:40.379213639Z}"
build_info::format!("{{{.crate_info.name} v{.crate_info.version} built with rustc version {.compiler.version} {.compiler.commit_hash} at {.timestamp}}}")
```

The [sample](sample) project shows both variants.

## Features
The `build_info` package supports several feature flags:
- The `runtime` feature enables `build_info::build_info!`. It is enabled by default, but if you intend to only use `build_info::format!`, it is safe to disable this flag.
- The `nested` feature adds support for [`proc-macro-nested`](https://crates.io/crates/proc-macro-nested), which lets the `build_info::format!` macro be nested inside other proc-macros. This may require you to set `#![recursion_limit = "..."]` in your crate. The feature is disabled by default.
- The `chrono` feature enables the default features of the `chrono` package, which is used by `build_info::build_info!`. It is disabled by default.
- The `serde` feature adds `Serialize`/`Deserialize` support to the types used by `build_info::build_info!`. It is disabled by default.

# Caveats
As of the time of writing, Rust does not support function-like proc-macros used as expressions.
The `format!` macro can often still be used as an expression, thanks to [the `proc-macro-hack` crate](https://crates.io/crates/proc-macro-hack).
However, its result will not behave like a string literal in all cases; for example, it cannot be used as an argument to `concat!`.

The build script will ask cargo to rerun it whenever the project or the currently checked out commit changes.
It will not necessarily be rerun if only the dependencies change (`build_info_build::build_script` will try to find the lockfile and depend on it, but it is not really aware of any of the more intricate features, such as, cargo workspaces).
Please open an issue if your specific use case requires a more strict rerun policy for `build.rs` and include a short description what additional files should trigger a rebuild when changed.
