impl std::fmt::Display for crate::BuildInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} {} build", self.crate_info, self.profile)?;

		if let Some(crate::VersionControl::Git(ref git)) = self.version_control {
			write!(f, " from {git}")?;
		}

		Ok(())
	}
}

impl std::fmt::Display for crate::OptimizationLevel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::O0 => write!(f, "0"),
			Self::O1 => write!(f, "1"),
			Self::O2 => write!(f, "2"),
			Self::O3 => write!(f, "3"),
			Self::Os => write!(f, "s"),
			Self::Oz => write!(f, "z"),
		}
	}
}

impl std::fmt::Display for crate::CrateInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} v{}", self.name, self.version)
	}
}

impl std::fmt::Display for crate::CompilerInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "rustc {}", self.version)?;

		if let Some(ref commit_id) = self.commit_id {
			let commit_id = &commit_id[0..9];
			if let Some(ref commit_date) = self.commit_date {
				write!(f, " ({commit_id} {commit_date})")?;
			} else {
				write!(f, " ({commit_id})")?;
			}
		}

		Ok(())
	}
}

impl std::fmt::Display for crate::VersionControl {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			crate::VersionControl::Git(ref git) => write!(f, "{git}"),
		}
	}
}

impl std::fmt::Display for crate::GitInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", &self.commit_id)?;

		if self.dirty {
			write!(f, ".+")?;
		}

		if let Some(branch) = &self.branch {
			write!(f, " ({branch})")?;
		}

		Ok(())
	}
}
