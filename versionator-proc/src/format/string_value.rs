use versionator_common::{
	BuildInfo, CompilerChannel, CompilerVersion, GitInformation, Identifier, Version, VersionControl,
};

use std::collections::VecDeque;

pub(crate) fn string_value<T: StringValue>(this: &T, indeces: VecDeque<String>) -> String {
	this.string_value(indeces)
}

pub(crate) trait StringValue {
	fn string_value(&self, indeces: VecDeque<String>) -> String;
}

impl StringValue for BuildInfo {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			unimplemented!();
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"compiler" => string_value(&self.compiler, indeces),
			"version_control" => string_value(&self.version_control, indeces),
			_ => panic!(format!("The member {} is not valid for versionator::BuildInfo", index)),
		}
	}
}

impl<T: StringValue> StringValue for Option<T> {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			if let Some(value) = self {
				return string_value(value, indeces);
			} else {
				return "UNKNOWN".to_string();
			}
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"?" => match self {
				Some(value) => string_value(value, indeces),
				None => panic!("Failed to unwrap element"),
			},
			_ => panic!(format!("The member {} is not valid for Option<T>", index)),
		}
	}
}

impl StringValue for CompilerVersion {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			unimplemented!();
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"version" => string_value(&self.version, indeces),
			"commit_hash" => string_value(&self.commit_hash, indeces),
			"commit_date" => string_value(&self.commit_date, indeces),
			"channel" => string_value(&self.channel, indeces),
			"host_triple" => string_value(&self.host_triple, indeces),
			"target_triple" => string_value(&self.target_triple, indeces),
			_ => panic!(format!(
				"The member {} is not valid for versionator::CompilerVersion",
				index
			)),
		}
	}
}

impl StringValue for Version {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return self.to_string();
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"major" => string_value(&self.major, indeces),
			"minor" => string_value(&self.minor, indeces),
			"patch" => string_value(&self.patch, indeces),
			"pre" => string_value(&self.pre, indeces),
			"build" => string_value(&self.build, indeces),
			_ => panic!(format!(
				"The member {} is not valid for versionator::CompilerVersion",
				index
			)),
		}
	}
}

impl StringValue for bool {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return format!("{}", self);
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("The member {} is not valid for bool", index));
	}
}

impl StringValue for u64 {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return format!("{}", self);
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("The member {} is not valid for u64", index));
	}
}

impl StringValue for String {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			return self.to_string();
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("The member {} is not valid for String", index));
	}
}

impl<T: StringValue> StringValue for Vec<T> {
	fn string_value(&self, _indeces: VecDeque<String>) -> String {
		unimplemented!();
	}
}

impl StringValue for Identifier {
	fn string_value(&self, _indeces: VecDeque<String>) -> String {
		unimplemented!();
	}
}

impl StringValue for CompilerChannel {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
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

impl StringValue for VersionControl {
	fn string_value(&self, indeces: VecDeque<String>) -> String {
		match self {
			versionator_common::VersionControl::Git(value) => string_value(value, indeces),
		}
	}
}

impl StringValue for GitInformation {
	fn string_value(&self, mut indeces: VecDeque<String>) -> String {
		if indeces.is_empty() {
			let dirty = if self.dirty { "+" } else { "" };
			if let Some(name) = &self.name {
				return format!("{}{} ({}{})", name, dirty, &self.commit_hash, dirty);
			} else {
				return format!("{}{}", &self.commit_hash, dirty);
			}
		}

		let index = indeces.pop_front().unwrap();
		match index.as_ref() {
			"commit_hash" => string_value(&self.commit_hash, indeces),
			"dirty" => string_value(&self.dirty, indeces),
			"name" => string_value(&self.name, indeces),
			_ => panic!(format!(
				"The member {} is not valid for versionator::GitInformation",
				index
			)),
		}
	}
}
