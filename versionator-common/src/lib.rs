#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version<'a> {
	pub compiler: CompilerVersion<'a>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompilerVersion<'a> {
	pub channel: CompilerChannel,
	pub host_triple: &'a str,
	pub target_triple: &'a str,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}
