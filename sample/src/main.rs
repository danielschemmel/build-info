use versionator::version;

versionator::versionator!(fn full_version);

fn main() {
	println!("{:?}", full_version());
	println!("{:?}", version!(.version_control?.name));
	println!("{:?}", version!(.compiler.commit_hash?));
	println!("{:?}", version!(.compiler.version.pre));
}
