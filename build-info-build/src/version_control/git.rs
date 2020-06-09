use anyhow::{anyhow, Result};

use build_info_common::chrono::{TimeZone, Utc};
use build_info_common::GitInfo;

use git2::{Oid, Repository, StatusOptions};

pub(crate) fn get_info() -> Result<GitInfo> {
	let repository = Repository::discover(".")?;
	println!(
		"cargo:rerun-if-changed={}",
		repository.path().join("HEAD").to_str().unwrap()
	);

	let head = repository.head()?;
	if let Some(name) = head.name() {
		// HEAD has already been added
		if name != "HEAD" {
			let path = repository.path().join(name);
			if path.is_file() {
				println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
			} else {
				println!(
					"cargo:rerun-if-changed={}",
					repository.path().join("packed-refs").to_str().unwrap()
				);
			}
		}
	}
	let commit = head.peel_to_commit()?;
	let commit_id = commit.id();
	let commit_short_id = commit.as_object().short_id()?.as_str().unwrap().to_string();
	let commit_timestamp = Utc.timestamp(commit.time().seconds(), 0);

	let changes = repository.statuses(Some(StatusOptions::new().include_ignored(false)))?;
	let dirty = !changes.is_empty();

	let tags = tags(&repository, &commit_id)?;

	Ok(GitInfo {
		commit_id: commit_id.to_string(),
		commit_short_id,
		commit_timestamp,
		dirty,
		branch: if head.is_branch() {
			head.shorthand().map(|s| s.to_string())
		} else {
			None
		},
		tags,
	})
}

const TAGS_PREFIX: &str = "refs/tags/";

fn tags(repository: &Repository, commit_id: &Oid) -> Result<Vec<String>> {
	let mut result = Vec::new();
	for reference in repository.references()? {
		let reference = reference?;
		if reference.is_tag() {
			let referenced_commit = reference.peel_to_commit()?;
			if referenced_commit.id() == *commit_id {
				let name = reference.name().ok_or_else(|| anyhow!("Encountered a tag without a UTF-8 compatible name"))?;
				if name.starts_with(TAGS_PREFIX) {
					result.push(name[TAGS_PREFIX.len()..].to_string());
				} else {
					return Err(anyhow!("Encountered tag that does not begin with {:?}: {:?}", TAGS_PREFIX, name));
				}
			}
		}
	}
	Ok(result)
}
