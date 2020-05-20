pub use lazy_static::lazy_static; // used by the proc macro
pub use versionator_proc::versionator;

pub use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BuildInfo {
	pub compiler: CompilerVersion,
	pub version_control: Option<VersionControl>,
}

impl BuildInfo {
	pub fn serialize(&self) -> String {
		serde_json::to_string(self).unwrap()
	}

	pub fn deserialize(value: &str) -> Self {
		serde_json::from_str(value).unwrap()
	}
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompilerVersion {
	pub version: Version,
	pub commit_hash: Option<String>,
	pub commit_date: Option<String>,
	pub channel: CompilerChannel,
	pub host_triple: String,
	pub target_triple: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionControl {
	Git {
		commit_hash: String,
		dirty: bool,
		name: Option<String>,
	},
}
