use build_info_common::VersionControl;

#[cfg(feature = "git")]
mod git;

#[cfg(feature = "git")]
fn get_git_info() -> anyhow::Result<VersionControl> {
	git::get_info().map(VersionControl::Git)
}

#[cfg(not(feature = "git"))]
fn get_git_info() -> anyhow::Result<VersionControl> {
	Err(anyhow::anyhow!("Git support is disabled"))
}

pub fn get_info() -> Option<VersionControl> {
	if cfg!(feature = "git") {
		if let Ok(info) = get_git_info() {
			return Some(info);
		}
	}

	None
}
