use versionator::BuildInfo;

mod cargo;
mod compiler;
mod version_control;

pub fn build_script() {
	let crate_info = cargo::read();
	let compiler = compiler::get_info();
	let version_control = version_control::get_info();

	let timestamp = versionator::Utc::now();
	cargo::read();
	let build_info = BuildInfo {
		timestamp,
		crate_info,
		compiler,
		version_control,
	};

	println!("cargo:rustc-env=VERSIONATOR={}", build_info.serialize());
}
