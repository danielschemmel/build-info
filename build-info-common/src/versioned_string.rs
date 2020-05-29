use serde::{Deserialize, Serialize};

use crate::{crate_version, Version};

/// Used internally to ensure that `build-info` and `build-info-build` use the same version of `build-info-common`.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VersionedString {
	pub version: Version,
	pub string: String,
}

impl VersionedString {
	pub fn build_info_common_versioned(string: String) -> Self {
		Self {
			version: crate_version(),
			string,
		}
	}

	pub fn check(&self) -> bool {
		self.version == Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
	}
}
