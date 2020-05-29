// Use the `build_info!` macro to generate a function `crate::build_info` that returns on object with the data that
// is collected in the build script.
// This macro supports visibility-specifiers, like `build_info!(pub fn how_this_crate_was_built)`.
build_info::build_info!(fn build_info);

fn main() {
	// We can now either use the `build_info` function to work with the collected data at runtime...
	println!("{:#?}", build_info());

	// ... or format it directly to a single `&'static str` at compile time
	println!(
		"{}",
		build_info::format!("{{{.crate_info.name} v{.crate_info.version} built with rustc version {.compiler.version} {.compiler.commit_hash} at {.timestamp}}}")
	);
}
