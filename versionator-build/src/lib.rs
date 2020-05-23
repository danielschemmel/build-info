use versionator::BuildInfo;

mod compiler;
mod crate_info;
mod version_control;

fn rebuild_if_project_changes() {
	println!("cargo:rerun-if-changed={}", "Cargo.toml");
	for source in glob::glob_with(
		"**/*.rs",
		glob::MatchOptions {
			case_sensitive: false,
			require_literal_separator: false,
			require_literal_leading_dot: false,
		},
	).unwrap() {
		let source = source.unwrap();
		println!("cargo:rerun-if-changed={}", source.to_string_lossy());
	}
}

pub fn build_script() {
	rebuild_if_project_changes();

	let crate_info = crate_info::read_manifest();
	let compiler = compiler::get_info();
	let version_control = version_control::get_info();

	let timestamp = versionator::Utc::now();
	let build_info = BuildInfo {
		timestamp,
		crate_info,
		compiler,
		version_control,
	};

	println!(
		"cargo:rustc-env=VERSIONATOR={}",
		serde_json::to_string(&build_info).unwrap()
	);
}
