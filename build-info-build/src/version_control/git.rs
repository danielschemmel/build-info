use anyhow::{anyhow, Result};

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

	let changes = repository.statuses(Some(StatusOptions::new().include_ignored(false)))?;
	let dirty = !changes.is_empty();

	let tags = tags(&repository, &commit_id)?;

	Ok(GitInfo {
		commit_id: commit_id.to_string(),
		dirty,
		branch: if head.is_branch() { head.shorthand().map(|s| s.to_string()) } else { None },
		tags,
	})
}

fn tags(repository: &Repository, commit_id: &Oid) -> Result<Vec<String>> {
	repository
		.references()?
		.map(|r| r.map_err(|e| e.into()))
		.filter(|r| r.as_ref().map(|r| r.is_tag()).unwrap_or(true)) // keep all errors around
		.map(|r| r.and_then(|r| r.peel_to_commit().map(|c| (r, c)).map_err(|e| e.into())))
		.filter(|t| t.as_ref().map(|(_r, c)| c.id() == *commit_id).unwrap_or(true)) // keep all errors around
		.map(|t| t.map(|(r, _c)| r.name().map(|name| name.to_string())))
		.map(|o| o.and_then(|name| name.ok_or_else(|| anyhow!("Encountered unnamed tag"))))
		.collect()
}
