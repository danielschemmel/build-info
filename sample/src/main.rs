// use versionator::versionator_static;

versionator::versionator!(fn version);

static VERSION: &'static str = concat!("sample ");

fn main() {
	println!("{}", VERSION);
	println!("{:?}", version());
}
