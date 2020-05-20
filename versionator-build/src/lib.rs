use versionator_common::BuildInfo;

mod compiler;
mod version_control;

pub fn build_script() {
	let build_info = BuildInfo {
		compiler: compiler::get_info(),
		version_control: version_control::get_info(),
	};

	println!(
		"cargo:rustc-env=VERSIONATOR={}",
		serde_json::to_string(&build_info).unwrap()
	);
}
