fn main() {
	println!("cargo:rustc-env=HOST_TRIPLE={}", std::env::var("HOST").unwrap());
	println!("cargo:rustc-env=TARGET_TRIPLE={}", std::env::var("TARGET").unwrap());
}
