use anyhow::{anyhow, Result};
use git2::{Repository, StatusOptions};

use versionator::{GitInformation, VersionControl};

fn get_git_info() -> Result<GitInformation> {
	let repo = Repository::discover(".")?;
	println!("cargo:rerun-if-changed={}", repo.path().join("HEAD").to_str().unwrap());

	let head = repo.head()?;
	if let Some(name) = head.name() {
		if name != "HEAD" {
			// already added
			println!("cargo:rerun-if-changed={}", repo.path().join("logs").join(name).to_str().unwrap());
		}
	}
	let commit_hash = head
		.target()
		.ok_or_else(|| anyhow!("Could not unwrap the commit hash"))?
		.to_string();

	let changes = repo.statuses(Some(StatusOptions::new().include_ignored(false)))?;
	let dirty = !changes.is_empty();

	Ok(GitInformation {
		commit_hash,
		dirty,
		name: head.shorthand().map(|s| s.to_string()),
	})
}

pub fn get_info() -> Option<VersionControl> {
	if let Ok(info) = get_git_info() {
		return Some(VersionControl::Git(info));
	}

	None
}
