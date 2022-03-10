/*!
Common types used by the `build-info` and `build-info-build` crates.

User code should not depend on this crate directly, but rather depend on `build-info` (as a `[dependency]`) and `build-info-build` (as a `[build-dependency]`).
The types provided herein are reexported by `build-info` and should be used that way.
For example, `build_info_common::BuildInfo` should be used as `build_info::BuildInfo` instead.
*/

#![forbid(unsafe_code)]

use derive_more::Display;

pub use chrono;
use chrono::{DateTime, NaiveDate, Utc};

pub use semver;
use semver::Version;

#[cfg(feature = "enable-pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "enable-serde")]
mod versioned_string;
#[cfg(feature = "enable-serde")]
pub use versioned_string::VersionedString;

pub mod display;

/// Gets the version of the `build-info-common` crate (this crate)
pub fn crate_version() -> Version {
	Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

/// Information about the current build
#[cfg_attr(feature = "enable-pyo3", pyclass)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BuildInfo {
	/// Updated whenever `build.rs` is rerun.
	pub timestamp: DateTime<Utc>,

	/// Cargo currently supports two different build types: `"Release"` and `"Debug"`
	pub profile: String,

	/// The optimization level can be set in `Cargo.toml` for each profile
	pub optimization_level: OptimizationLevel,

	/// Information about the current crate
	pub crate_info: CrateInfo,

	/// Information about the compiler used.
	pub compiler: CompilerInfo,

	/// `Some` if the project is inside a check-out of a supported version control system.
	pub version_control: Option<VersionControl>,
}

#[cfg(feature = "enable-pyo3")]
#[pymethods]
impl BuildInfo {
	fn __str__(&self) -> String {
		format!("{}", self)
	}

	fn __repr__(&self) -> String {
		format!("{:?}", self)
	}

	/// Gets *almost* the timestamp, as Python's `datetime` does not account for leap seconds
	#[getter]
	fn timestamp<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
		use chrono::{Datelike, Timelike};

		let py_datetime = py.import("datetime")?;
		py_datetime.getattr("datetime")?.call1((
			self.timestamp.year_ce().1,
			self.timestamp.month(),
			self.timestamp.day(),
			self.timestamp.hour(),
			self.timestamp.minute(),
			self.timestamp.second(),
			self.timestamp.timestamp_subsec_micros() % 1_000_000,
			py_datetime.getattr("timezone")?.getattr("utc")?,
		))
	}

	#[getter]
	fn profile(&self) -> &str {
		&self.profile
	}

	#[getter]
	fn optimization_level(&self) -> OptimizationLevel {
		self.optimization_level
	}

	#[getter]
	fn crate_info(&self) -> CrateInfo {
		self.crate_info.clone()
	}

	#[getter]
	fn compiler(&self) -> CompilerInfo {
		self.compiler.clone()
	}

	#[getter]
	fn version_control<'py>(&self, py: Python<'py>) -> Py<PyAny> {
		match self.version_control {
			Some(VersionControl::Git(ref git)) => git.clone().into_py(py),
			None => py.None(),
		}
	}
}

/// The various possible optimization levels
#[cfg_attr(feature = "enable-pyo3", pyclass)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum OptimizationLevel {
	O0,
	O1,
	O2,
	O3,
	Os,
	Oz,
}

/// Information about the current crate (i.e., the crate for which build information has been generated)
#[cfg_attr(feature = "enable-pyo3", pyclass)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CrateInfo {
	/// The name, as defined in `Cargo.toml`.
	pub name: String,

	/// The version, as defined in `Cargo.toml`.
	pub version: Version,

	/// The authors, as defined in `Cargo.toml`.
	pub authors: Vec<String>,

	/// The license string, as defined in `Cargo.toml`.
	pub license: Option<String>,

	/// The features of this crate that are currently enabled in this configuration.
	pub enabled_features: Vec<String>,

	/// All features that are available from this crate.
	pub available_features: Vec<String>,

	/// Dependencies of this crate.
	/// Will only be filled with data if `build-info-build` has the `dependencies` feature enabled.
	pub dependencies: Vec<CrateInfo>,
}

#[cfg(feature = "enable-pyo3")]
#[pymethods]
impl CrateInfo {
	fn __str__(&self) -> String {
		format!("{}", self)
	}

	fn __repr__(&self) -> String {
		format!("{:?}", self)
	}

	#[getter]
	fn name(&self) -> &str {
		&self.name
	}

	#[getter]
	fn version(&self) -> String {
		self.version.to_string()
	}

	#[getter]
	fn authors(&self) -> Vec<&str> {
		self.authors.iter().map(|s| s as &str).collect()
	}

	#[getter]
	fn license<'py>(&self) -> Option<&str> {
		self.license.as_ref().map(|s| s as &str)
	}

	#[getter]
	fn enabled_features(&self) -> Vec<&str> {
		self.enabled_features.iter().map(|s| s as &str).collect()
	}

	#[getter]
	fn available_features(&self) -> Vec<&str> {
		self.available_features.iter().map(|s| s as &str).collect()
	}

	#[getter]
	fn dependencies(&self) -> Vec<CrateInfo> {
		self.dependencies.clone()
	}
}

/// `rustc` version and configuration
#[cfg_attr(feature = "enable-pyo3", pyclass)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompilerInfo {
	/// Version of the current `rustc`
	pub version: Version,

	/// Commit hash from which `rustc` was built
	pub commit_id: Option<String>,

	/// Date on which `rustc` was built
	pub commit_date: Option<NaiveDate>,

	/// Channel which was configured for this version of `rustc`
	pub channel: CompilerChannel,

	/// Identifies the host on which `rustc` was running
	pub host_triple: String,

	/// Identifies the target architecture for which the crate is being compiled
	pub target_triple: String,
}

#[cfg(feature = "enable-pyo3")]
#[pymethods]
impl CompilerInfo {
	fn __str__(&self) -> String {
		format!("{}", self)
	}

	fn __repr__(&self) -> String {
		format!("{:?}", self)
	}

	#[getter]
	fn version(&self) -> String {
		self.version.to_string()
	}

	#[getter]
	fn commit_id(&self) -> Option<&str> {
		self.commit_id.as_ref().map(|s| s as &str)
	}

	#[getter]
	fn commit_date<'py>(&self, py: Python<'py>) -> PyResult<Option<&'py PyAny>> {
		use chrono::Datelike;

		self
			.commit_date
			.map(|ref date| {
				py.import("datetime")?
					.getattr("date")?
					.call1((date.year_ce().1, date.month(), date.day()))
			})
			.transpose()
	}

	#[getter]
	fn channel(&self) -> CompilerChannel {
		self.channel
	}

	#[getter]
	fn host_triple(&self) -> &str {
		&self.host_triple
	}

	#[getter]
	fn target_triple(&self) -> &str {
		&self.target_triple
	}
}

/// `rustc` distribution channel (some compiler features are only available on specific channels)
#[cfg_attr(feature = "enable-pyo3", pyclass)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Display, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}

/// Support for different version control systems
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionControl {
	Git(GitInfo),
}

impl VersionControl {
	pub fn git(&self) -> Option<&GitInfo> {
		match self {
			VersionControl::Git(git) => Some(git),
			// _ => None, // Pattern currently unreachable
		}
	}
}

/**
Information about a git repository

If a git repository is detected (and, thereby, this information included), the build script will be rerun whenever the
currently checked out commit changes.
*/
#[cfg_attr(feature = "enable-pyo3", pyclass)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitInfo {
	/// Full commit hash for the currently checked out commit
	pub commit_id: String,

	/// Short commit hash for the currently checked out commit
	///
	/// The length of this string depends on the effective value of the git configuration variable `core.abbrev`, and is
	/// extended to the minimum length required for the id to be unique (at the time it was computed).
	pub commit_short_id: String,

	/// Timestamp of the currently checked out commit
	pub commit_timestamp: DateTime<Utc>,

	/// `true` iff the repository had uncommitted changes when building the project.
	pub dirty: bool,

	/// Names the branch that is currently checked out, if any
	pub branch: Option<String>,

	/// All tags that point to the current commit (e.g., `["v0.0.10", "sample@v0.0.10"]`)
	pub tags: Vec<String>,
}

#[cfg(feature = "enable-pyo3")]
#[pymethods]
impl GitInfo {
	fn __str__(&self) -> String {
		format!("{}", self)
	}

	fn __repr__(&self) -> String {
		format!("{:?}", self)
	}

	#[getter]
	fn commit_id(&self) -> &str {
		&self.commit_id
	}

	#[getter]
	fn commit_short_id(&self) -> &str {
		&self.commit_short_id
	}

	/// Gets *almost* the timestamp, as Python's `datetime` does not account for leap seconds
	#[getter]
	fn commit_timestamp<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
		use chrono::{Datelike, Timelike};

		let py_datetime = py.import("datetime")?;
		py_datetime.getattr("datetime")?.call1((
			self.commit_timestamp.year_ce().1,
			self.commit_timestamp.month(),
			self.commit_timestamp.day(),
			self.commit_timestamp.hour(),
			self.commit_timestamp.minute(),
			self.commit_timestamp.second(),
			self.commit_timestamp.timestamp_subsec_micros() % 1_000_000,
			py_datetime.getattr("timezone")?.getattr("utc")?,
		))
	}

	#[getter]
	fn dirty(&self) -> bool {
		self.dirty
	}

	#[getter]
	fn branch<'py>(&self) -> Option<&str> {
		self.branch.as_ref().map(|s| s as &str)
	}

	#[getter]
	fn tags(&self) -> Vec<&str> {
		self.tags.iter().map(|s| s as &str).collect()
	}
}
