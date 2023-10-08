use build_info_build::DependencyDepth;

fn main() {
	// Calling `build_info_build::build_script` collects all data and makes it available to `build_info::build_info!`
	// and `build_info::format!` in the main program.
	//
	// Dependency collection needs to be enabled specifically.
	build_info_build::build_script().collect_dependencies(DependencyDepth::Full);
}
