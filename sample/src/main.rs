versionator::versionator!(fn version);

fn main() {
	println!("{:?}", version());
	println!(
		"{}",
		versionator::format!("{{{.crate_info.name} v{.crate_info.version} built with rustc version {.compiler.version} {.compiler.commit_hash} at {.timestamp}}}")
	);
}
