use std::collections::VecDeque;

use versionator_common::{
	BuildInfo, CompilerChannel, CompilerVersion, CrateInfo, DateTime, GitInformation, Identifier, Utc, Version,
	VersionControl,
};

pub(crate) fn indexed_string_value<T: IndexedStringValue>(this: &T, indeces: VecDeque<String>) -> String {
	this.indexed_string_value(indeces)
}

pub(crate) trait IndexedStringValue {
	fn indexed_string_value(&self, indeces: VecDeque<String>) -> String;
}

impl IndexedStringValue for BuildInfo {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			unimplemented!();
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"timestamp" => indexed_string_value(&self.timestamp, indeces),
			"crate_info" => indexed_string_value(&self.crate_info, indeces),
			"compiler" => indexed_string_value(&self.compiler, indeces),
			"version_control" => indexed_string_value(&self.version_control, indeces),
			_ => panic!(format!("The member {} is not valid for versionator::BuildInfo", index)),
		}
	}
}

impl IndexedStringValue for CrateInfo {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			unimplemented!();
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"name" => indexed_string_value(&self.name, indeces),
			"version" => indexed_string_value(&self.version, indeces),
			_ => panic!(format!("The member {} is not valid for versionator::BuildInfo", index)),
		}
	}
}

impl IndexedStringValue for DateTime<Utc> {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return self.format("%Y-%m-%d %H:%M:%S%.fZ").to_string();
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("The member {} is not valid for DateTime<Utc>", index));
	}
}

impl<T: IndexedStringValue> IndexedStringValue for Option<T> {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			if let Some(value) = self {
				return indexed_string_value(value, indeces);
			} else {
				return "UNKNOWN".to_string();
			}
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"?" => match self {
				Some(value) => indexed_string_value(value, indeces),
				None => panic!("Failed to unwrap element"),
			},
			_ => panic!(format!("The member {} is not valid for Option<T>", index)),
		}
	}
}

impl IndexedStringValue for CompilerVersion {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			unimplemented!();
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"version" => indexed_string_value(&self.version, indeces),
			"commit_hash" => indexed_string_value(&self.commit_hash, indeces),
			"commit_date" => indexed_string_value(&self.commit_date, indeces),
			"channel" => indexed_string_value(&self.channel, indeces),
			"host_triple" => indexed_string_value(&self.host_triple, indeces),
			"target_triple" => indexed_string_value(&self.target_triple, indeces),
			_ => panic!(format!(
				"The member {} is not valid for versionator::CompilerVersion",
				index
			)),
		}
	}
}

impl IndexedStringValue for Version {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return self.to_string();
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"major" => indexed_string_value(&self.major, indeces),
			"minor" => indexed_string_value(&self.minor, indeces),
			"patch" => indexed_string_value(&self.patch, indeces),
			"pre" => indexed_string_value(&self.pre, indeces),
			"build" => indexed_string_value(&self.build, indeces),
			_ => panic!(format!(
				"The member {} is not valid for versionator::CompilerVersion",
				index
			)),
		}
	}
}

impl IndexedStringValue for bool {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return format!("{}", self);
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("The member {} is not valid for bool", index));
	}
}

impl IndexedStringValue for u64 {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return format!("{}", self);
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("The member {} is not valid for u64", index));
	}
}

impl IndexedStringValue for String {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return self.to_string();
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("The member {} is not valid for String", index));
	}
}

impl<T: IndexedStringValue> IndexedStringValue for Vec<T> {
	fn indexed_string_value(&self, _indeces: VecDeque<String>) -> String {
		unimplemented!();
	}
}

impl IndexedStringValue for Identifier {
	fn indexed_string_value(&self, _indeces: VecDeque<String>) -> String {
		unimplemented!();
	}
}

impl IndexedStringValue for CompilerChannel {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return self.to_string();
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!(
			"The member {} is not valid for versionator::CompilerChannel",
			index
		));
	}
}

impl IndexedStringValue for VersionControl {
	fn indexed_string_value(&self, indeces: VecDeque<String>) -> String {
		match self {
			versionator_common::VersionControl::Git(value) => indexed_string_value(value, indeces),
		}
	}
}

impl IndexedStringValue for GitInformation {
	fn indexed_string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			let dirty = if self.dirty { "+" } else { "" };
			if let Some(name) = &self.name {
				return format!("{}{} ({})", &self.commit_hash, dirty, name);
			} else {
				return format!("{}{}", &self.commit_hash, dirty);
			}
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"commit_hash" => indexed_string_value(&self.commit_hash, indeces),
			"dirty" => indexed_string_value(&self.dirty, indeces),
			"name" => indexed_string_value(&self.name, indeces),
			_ => panic!(format!(
				"The member {} is not valid for versionator::GitInformation",
				index
			)),
		}
	}
}
