use anyhow::{anyhow, Result};
use git2::{Repository, StatusOptions};

use versionator_common::VersionControl;

fn get_git_info() -> Result<VersionControl> {
	let repo = Repository::discover(".")?;
	println!("cargo:rerun-if-changed={}", repo.path().join("HEAD").to_str().unwrap());

	let head = repo.head()?;
	if head.name() != None && head.name() != Some("HEAD") {
		println!("cargo:rerun-if-changed={}", repo.path().join("refs").join(head.name().unwrap()).to_str().unwrap());
	}
	let commit_hash = head
		.target()
		.ok_or_else(|| anyhow!("Could not unwrap the commit hash"))?
		.to_string();

	let changes = repo.statuses(Some(StatusOptions::new().include_ignored(false)))?;
	let dirty = !changes.is_empty();

	Ok(VersionControl::Git {
		commit_hash: commit_hash,
		dirty: dirty,
		name: head.shorthand().map(|s| s.to_string()),
	})
}

pub fn get_info() -> Option<VersionControl> {
	if let Ok(info) = get_git_info() {
		return Some(info);
	}

	None
}
