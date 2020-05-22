use versionator::version;

versionator::versionator!(fn full_version);

fn main() {
	println!("{:?}", full_version());
	println!("{:?}", version!(.version_control?.name));
	println!("{:?}", version!(.compiler.commit_hash?));
	println!("{:?}", version!(.compiler.channel.to_string()));
	println!("{:?}", version!(.compiler.version.to_string()));
	println!(
		"{}",
		versionator::format!("{{Built with rustc version {.compiler.version} {.compiler.commit_hash} at {.timestamp}}}")
	);
}
