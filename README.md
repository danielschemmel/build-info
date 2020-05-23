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
