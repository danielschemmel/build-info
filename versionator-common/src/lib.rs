pub use chrono::{DateTime, TimeZone, Utc};
use derive_more::Display;
pub use semver::{Identifier, Version};
#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BuildInfo {
	pub timestamp: DateTime<Utc>,
	pub crate_info: CrateInfo,
	pub compiler: CompilerVersion,
	pub version_control: Option<VersionControl>,
}

pub fn nanos_to_utc(nanos: i64) -> DateTime<Utc> {
	Utc.timestamp_nanos(nanos)
}

#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CrateInfo {
	pub name: String,
	pub version: Version,
	pub authors: Vec<String>,
}

#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompilerVersion {
	pub version: Version,
	pub commit_hash: Option<String>,
	pub commit_date: Option<String>,
	pub channel: CompilerChannel,
	pub host_triple: String,
	pub target_triple: String,
}

#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Display, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}

#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionControl {
	Git(GitInformation),
}

#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitInformation {
	pub commit_hash: String,
	pub dirty: bool,
	pub name: Option<String>,
}
