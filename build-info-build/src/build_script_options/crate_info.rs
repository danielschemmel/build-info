use pretty_assertions::assert_eq;

use build_info_common::semver::Version;
use build_info_common::CrateInfo;

use std::path::Path;

pub(crate) fn read_manifest() -> CrateInfo {
	let meta = cargo_metadata::MetadataCommand::new()
		.cargo_path(std::env::var_os("CARGO").unwrap())
		.manifest_path(Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml"))
		.exec()
		.unwrap();
	let dependencies = meta.resolve.as_ref().unwrap();
	let root = &meta[&dependencies.root.as_ref().unwrap()];

	let name = root.name.clone();
	assert_eq!(name, std::env::var("CARGO_PKG_NAME").unwrap()); // sanity check...

	let version = Version::parse(&root.version.to_string()).unwrap();
	assert_eq!(version.to_string(), std::env::var("CARGO_PKG_VERSION").unwrap()); // sanity check...

	let authors = root.authors.clone();
	assert_eq!(authors.join(":"), std::env::var("CARGO_PKG_AUTHORS").unwrap()); // sanity check...

	CrateInfo { name, version, authors }
}
