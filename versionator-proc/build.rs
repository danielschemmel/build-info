fn main() {
	// We assume that the target triple of this crate is the same as that of the target program.
	// This is pretty much always the case, but not actually guaranteed.
	println!("cargo:rustc-env=TARGET_TRIPLE={}", std::env::var("TARGET").unwrap());
}
