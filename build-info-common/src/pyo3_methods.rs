use pyo3::prelude::*;

use crate::{BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, GitInfo, OptimizationLevel, VersionControl};

/// The function generated via `build_info::build_info!` returns a reference to a statically initialized object
/// (`&'static BuildInfo`). However, `pyo3` wants to move the result, which is of course impossible for a borrowed
/// object. To deal with this, we add a conversion for `&BuildInfo` that automatically clones the borrowed value.
impl IntoPy<PyObject> for &BuildInfo {
	fn into_py(self, py: Python) -> PyObject {
			self.clone().into_py(py)
	}
}

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
	fn version(&self) -> Version {
		Version(self.version.clone())
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

#[pymethods]
impl CompilerInfo {
	fn __str__(&self) -> String {
		format!("{}", self)
	}

	fn __repr__(&self) -> String {
		format!("{:?}", self)
	}

	#[getter]
	fn version(&self) -> Version {
		Version(self.version.clone())
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

#[pyclass]
pub struct Version(semver::Version);

#[pymethods]
impl Version {
	fn __str__(&self) -> String {
		format!("{}", self.0)
	}

	fn __repr__(&self) -> String {
		format!("{:?}", self.0)
	}

	#[getter]
	fn major(&self) -> u64 {
		self.0.major
	}

	#[getter]
	fn minor(&self) -> u64 {
		self.0.minor
	}

	#[getter]
	fn patch(&self) -> u64 {
		self.0.patch
	}

	#[getter]
	fn pre(&self) -> String {
		self.0.pre.to_string()
	}

	#[getter]
	fn build(&self) -> String {
		self.0.build.to_string()
	}
}
