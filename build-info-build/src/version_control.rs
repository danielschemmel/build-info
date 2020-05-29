use anyhow::{anyhow, Result};

#[cfg(feature = "git")]
use git2::{Repository, StatusOptions};

use build_info_common::{GitInfo, VersionControl};

#[cfg(feature = "git")]
fn get_git_info() -> Result<GitInfo> {
	let repo = Repository::discover(".")?;
	println!("cargo:rerun-if-changed={}", repo.path().join("HEAD").to_str().unwrap());

	let head = repo.head()?;
	if let Some(name) = head.name() {
		// HEAD has already been added
		if name != "HEAD" {
			let path = repo.path().join(name);
			if path.is_file() {
				println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
			} else {
				println!(
					"cargo:rerun-if-changed={}",
					repo.path().join("packed-refs").to_str().unwrap()
				);
			}
		}
	}
	let commit_hash = head
		.target()
		.ok_or_else(|| anyhow!("Could not unwrap the commit hash"))?
		.to_string();

	let changes = repo.statuses(Some(StatusOptions::new().include_ignored(false)))?;
	let dirty = !changes.is_empty();

	Ok(GitInfo {
		commit_hash,
		dirty,
		name: head.shorthand().map(|s| s.to_string()),
	})
}

#[cfg(not(feature = "git"))]
fn get_git_info() -> Result<GitInfo> {
	Err(anyhow!("Git support is disabled"))
}

pub fn get_info() -> Option<VersionControl> {
	if cfg!(feature = "git") {
		if let Ok(info) = get_git_info() {
			return Some(VersionControl::Git(info));
		}
	}

	None
}
