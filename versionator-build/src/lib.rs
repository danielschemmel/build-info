use versionator::BuildInfo;

mod compiler;
mod version_control;

pub fn build_script() {
	let build_info = BuildInfo {
		compiler: compiler::get_info(),
		version_control: version_control::get_info(),
	};

	println!(
		"cargo:rustc-env=VERSIONATOR={}",
		build_info.serialize()
	);
}
