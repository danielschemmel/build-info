# Usage
Begin by adding `versionator` as a `[dependency]` and `versionator-build` as a `[build-dependency]` to your [`Cargo.toml`](sample/Cargo.toml).
By separating those two crates, pure compile-time dependencies, such as `git2` are not compiled into your final program.

If it does not already exist, add a [`build.rs`](sample/build.rs) to your project's root, where you call `versionator_build::build_script()`.
This will collect build information at compile time.

Then, either use the `versionator!` macro to add a function that returns version information at runtime:
```rust
versionator::versionator!(fn version);
```
or use `versionator::format!` to generate a string at compile time:
```rust
versionator::format!("{{{.crate_info.name} v{.crate_info.version} built at {.timestamp}}}")
```

The [sample](sample) project shows both variants.

## Features
The `versionator` package supports several feature flags:
- The `runtime` feature enables `versionator::versionator!`. It is enabled by default. If you intend to only use `versionator::format!`, it is safe to disable this flag.
- The `chrono` feature enables the default features of the `chrono` package, which is used by `versionator::versionator!`. It is disabled by default.
- The `serde` feature adds `Serialize`/`Deserialize` support to the types used by `versionator::versionator!`. It is disabled by default.

# Caveats
As of the time of writing, Rust does not support function-like proc-macros used as expressions.
The `format!` macro can often still be used as an expression, thanks to [the `proc-macro-hack` crate](https://crates.io/crates/proc-macro-hack).
However, its result will not behave like a string literal in all cases; for example, it cannot be used as an argument to `concat!`.

The build script will ask cargo to rerun it whenever the project or the currently checked out commit changes.
It will not necessarily be rerun if only the dependencies change (`versionator_build::build_script` will try to find the lockfile and depend on it, but it is not really aware of any of the more intricate features, such as, cargo workspaces).
Please open an issue if your specific use case requires a more strict rerun policy for `build.rs` and include a short description what additional files should trigger a rebuild when changed.
