use pretty_assertions::assert_eq;

use build_info_common::semver::Version;
use build_info_common::CrateInfo;
use cargo_metadata::*;

use std::collections::hash_map::{Entry, HashMap};

impl crate::BuildScriptOptions {
	/// Enables and disables dependency collection.
	///
	/// Dependency data is fairly large, which may cause problems, mainly by crashing the build process. If the project
	/// compiles successfully with dependency collection enabled, you are probably fine.
	pub fn collect_dependencies(mut self, collect_dependencies: bool) -> Self {
		self.collect_dependencies = collect_dependencies;
		self
	}
}

pub(crate) struct Manifest {
	pub crate_info: CrateInfo,
	pub workspace_root: String,
}

pub(crate) fn read_manifest(target_platform: &str, collect_dependencies: bool) -> Manifest {
	let mut args = vec!["--filter-platform".to_string(), target_platform.to_string()];

	// Cargo does not provide a proper list of enabled features, so we collect metadata once to find all possible
	// features, convert them to the equivalent `CARGO_FEATURE_` representation, check for collisions, and then rerun
	// the command with the appropriate feature flags selected.
	//
	// We still expect this to fail for dependency flags that are enabled by hand (e.g.,
	// `cargo run --features=serde/derive`), but so far there is no workaround for that.

	let meta = MetadataCommand::new()
		.cargo_path(std::env::var_os("CARGO").unwrap())
		.manifest_path(&*super::CARGO_TOML)
		.features(CargoOpt::NoDefaultFeatures)
		.other_options(args.clone())
		.exec()
		.unwrap();

	let root = &meta[meta.resolve.as_ref().unwrap().root.as_ref().unwrap()];
	let mut map = HashMap::new();
	for feature in root.features.keys() {
		if !feature.is_ascii() {
			panic!("The feature {:?} contains non-ascii characters.", feature);
		}
		let env_var = format!("CARGO_FEATURE_{}", feature.to_ascii_uppercase().replace("-", "_"));
		if std::env::var_os(&env_var).is_some() {
			match map.entry(env_var) {
				Entry::Vacant(entry) => {
					entry.insert(feature);
				}
				Entry::Occupied(entry) => panic!(
					"The features {:?} and {:?} have the same representation as cargo feature flags ({:?})",
					feature,
					entry.get(),
					entry.key()
				),
			}
		}
	}

	let mut feature_list = String::new();
	for feature in map.values() {
		if !feature_list.is_empty() {
			feature_list += ",";
		}
		feature_list += feature;
	}
	args.push("--features".to_string());
	args.push(feature_list);

	let meta = MetadataCommand::new()
		.cargo_path(std::env::var_os("CARGO").unwrap())
		.manifest_path(&*super::CARGO_TOML)
		.features(CargoOpt::NoDefaultFeatures)
		.other_options(args)
		.exec()
		.unwrap();
	let crate_info = make_crate_info(&meta, collect_dependencies);

	assert_eq!(crate_info.name, std::env::var("CARGO_PKG_NAME").unwrap()); // sanity check...
	assert_eq!(
		crate_info.version.to_string(),
		std::env::var("CARGO_PKG_VERSION").unwrap()
	); // sanity check...
	assert_eq!(
		crate_info.authors.join(":"),
		std::env::var("CARGO_PKG_AUTHORS").unwrap()
	); // sanity check...

	Manifest {
		crate_info,
		workspace_root: meta.workspace_root.into(),
	}
}

fn make_crate_info(meta: &Metadata, collect_dependencies: bool) -> CrateInfo {
	let resolve = meta.resolve.as_ref().unwrap();
	let root_id = resolve.root.as_ref().unwrap();
	let dependencies: HashMap<&PackageId, &Node> = resolve.nodes.iter().map(|node| (&node.id, node)).collect();

	to_crate_info(dependencies[&root_id], &dependencies, meta, collect_dependencies)
}

fn to_crate_info(
	node: &Node,
	dependencies: &HashMap<&PackageId, &Node>,
	meta: &Metadata,
	collect_dependencies: bool,
) -> CrateInfo {
	let pkg = &meta[&node.id];
	let name = pkg.name.clone();
	let version = Version::parse(&pkg.version.to_string()).unwrap();
	let authors = pkg.authors.clone();
	let license = pkg.license.clone();
	let available_features = pkg.features.iter().map(|(key, _value)| key.clone()).collect();
	let enabled_features = node.features.clone();
	let dependencies = if collect_dependencies {
		node
			.deps
			.iter()
			.map(|dep| to_crate_info(dependencies[&dep.pkg], dependencies, meta, collect_dependencies))
			.collect()
	} else {
		Vec::new()
	};

	CrateInfo {
		name,
		version,
		authors,
		license,
		enabled_features,
		available_features,
		dependencies,
	}
}
