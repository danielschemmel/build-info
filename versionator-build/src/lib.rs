use versionator::BuildInfo;

mod compiler;
mod version_control;

pub fn build_script() {
	let now = versionator::Utc::now();

	let build_info = BuildInfo {
		timestamp: now,
		compiler: compiler::get_info(),
		version_control: version_control::get_info(),
	};

	println!("cargo:rustc-env=VERSIONATOR={}", build_info.serialize());
}
