use versionator::versionator;

versionator!(VERSION);

fn main() {
	println!("{:?}", VERSION);
}
