use build_info_common::{CrateInfo, Version};
use toml::Value;

pub(crate) fn read_manifest() -> CrateInfo {
	let cargo_file = std::fs::read_to_string("Cargo.toml").expect("Could not open Cargo.toml");
	let cargo: Value = toml::from_str(&cargo_file).expect("Cargo.toml contains invalid TOML");

	let pkg = cargo
		.get("package")
		.expect("Could not find \"package\" table in Cargo.toml");

	let name = pkg
		.get("name")
		.expect("Could not find \"name\" field in Cargo.toml's [package] table")
		.as_str()
		.expect("Cargo.toml's package.name is not a string")
		.to_string();

	let version_string = pkg
		.get("version")
		.expect("Could not find \"version\" field in Cargo.toml's [package] table")
		.as_str()
		.expect("Cargo.toml's package.version is not a string")
		.to_string();
	let version = Version::parse(&version_string).unwrap();

	let authors: Vec<String> = pkg
		.get("authors")
		.expect("Could not find \"authors\" field in Cargo.toml's [package] table")
		.as_array()
		.expect("Cargo.toml's package.authors is not an array")
		.iter()
		.map(|s| {
			s.as_str()
				.expect("Cargo.toml's package.authors is not an array of strings")
				.to_string()
		})
		.collect();

	CrateInfo { name, version, authors }
}
