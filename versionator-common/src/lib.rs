pub use chrono::{DateTime, TimeZone, Utc};
use derive_more::Display;
pub use semver::{Identifier, Version};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BuildInfo {
	pub timestamp: DateTime<Utc>,
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

pub fn nanos_to_utc(nanos: i64) -> DateTime<Utc> {
	Utc.timestamp_nanos(nanos)
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

#[derive(Display, Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionControl {
	Git(GitInformation),
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitInformation {
	pub commit_hash: String,
	pub dirty: bool,
	pub name: Option<String>,
}
