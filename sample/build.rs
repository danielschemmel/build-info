fn main() {
	/// Calling `versionator_build::build_script` collects all data and makes it available to `versionator::versionator!`
	/// and `versionator::format!` in the main program.
	versionator_build::build_script();
}
