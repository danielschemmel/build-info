pub use semver::Version;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BuildInfo<'a> {
	pub compiler: CompilerVersion<'a>,
	pub version_control: Option<VersionControl<'a>>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompilerVersion<'a> {
	pub version_str: &'a str,
	pub commit_hash: Option<&'a str>,
	pub commit_date: Option<&'a str>,
	pub channel: CompilerChannel,
	pub host_triple: &'a str,
	pub target_triple: &'a str,
}

#[cfg(feature = "semver")]
impl<'a> CompilerVersion<'a> {
	pub fn version(&self) -> Version {
		Version::parse(self.version_str).unwrap()
	}
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionControl<'a> {
	Git {
		commit_hash: &'a str,
		dirty: bool,
		name: Option<&'a str>,
	},
}
