fn main() {
	println!("{}", env!("VERSIONATOR"));

	versionator_build::build_script();
}
