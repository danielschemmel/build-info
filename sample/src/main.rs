// Use the `versionator!` macro to generate a function `crate::build_info` that returns on object with the data that
// is collected in the build script.
versionator::versionator!(fn build_info);

fn main() {
	// We can now either use the `build_info` function to work with the collected data at runtime...
	println!("{:?}", build_info());

	// ... or format it directly to a single `&'static str` at compile time
	println!(
		"{}",
		versionator::format!("{{{.crate_info.name} v{.crate_info.version} built with rustc version {.compiler.version} {.compiler.commit_hash} at {.timestamp}}}")
	);
	println!(
		"{}",
		versionator::format!("{.compiler.version.pre.len()} :> {.compiler.version.pre}")
	);
}
