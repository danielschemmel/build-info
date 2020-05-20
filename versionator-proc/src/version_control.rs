use anyhow::{anyhow, Result};
use git2::{Repository, StatusOptions};
use proc_macro2::TokenStream;
use quote::quote;

fn get_git_info() -> Result<TokenStream> {
	let repo = Repository::discover(".")?;

	let head = repo.head()?;
	let commit_hash = head
		.target()
		.ok_or_else(|| anyhow!("Could not unwrap the commit hash"))?
		.to_string();
	
	let name = if let Some(name) = head.shorthand() {
		quote!(Some(#name))
	} else {
		quote!(None)
	};

	let changes = repo.statuses(Some(StatusOptions::new().include_ignored(false)))?;
	let dirty = !changes.is_empty();

	Ok(quote!(Some(versionator::VersionControl::Git{
		commit_hash: #commit_hash,
		dirty: #dirty,
		name: #name,
	})))
}

pub fn get_info() -> TokenStream {
	if let Ok(tokens) = get_git_info() {
		return tokens;
	}

	quote!(None)
}
