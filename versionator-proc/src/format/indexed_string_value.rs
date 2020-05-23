use format_buf::format;

use std::collections::VecDeque;

use versionator_common::{
	BuildInfo, CompilerChannel, CompilerVersion, CrateInfo, DateTime, GitInformation, Identifier, Utc, Version,
	VersionControl,
};

use super::Index;

pub(crate) fn indexed_string_value<T: IndexedStringValue>(this: &T, indeces: VecDeque<Index>) -> String {
	this.indexed_string_value(indeces)
}

pub(crate) trait IndexedStringValue {
	fn indexed_string_value(&self, indeces: VecDeque<Index>) -> String;
}

impl IndexedStringValue for BuildInfo {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			unimplemented!();
		}

		let index = indeces.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "timestamp" => indexed_string_value(&self.timestamp, indeces),
			Index::Field(ref id) if id == "crate_info" => indexed_string_value(&self.crate_info, indeces),
			Index::Field(ref id) if id == "compiler" => indexed_string_value(&self.compiler, indeces),
			Index::Field(ref id) if id == "version_control" => indexed_string_value(&self.version_control, indeces),
			_ => panic!(format!("{:?} is not valid on versionator::BuildInfo", index)),
		}
	}
}

impl IndexedStringValue for CrateInfo {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			unimplemented!();
		}

		let index = indeces.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "name" => indexed_string_value(&self.name, indeces),
			Index::Field(ref id) if id == "version" => indexed_string_value(&self.version, indeces),
			Index::Field(ref id) if id == "authors" => indexed_string_value(&self.authors, indeces),
			_ => panic!(format!("{:?} is not valid for versionator::CrateInfo", index)),
		}
	}
}

impl IndexedStringValue for DateTime<Utc> {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			return self.format("%Y-%m-%d %H:%M:%S%.fZ").to_string();
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("{:?} is not valid for DateTime<Utc>", index));
	}
}

impl<T: IndexedStringValue> IndexedStringValue for Option<T> {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			if let Some(value) = self {
				return indexed_string_value(value, indeces);
			} else {
				return "UNKNOWN".to_string();
			}
		}

		let index = indeces.pop_front().unwrap();
		match index {
			Index::Unwrap => match self {
				Some(value) => indexed_string_value(value, indeces),
				None => panic!("Failed to unwrap element"),
			},
			_ => panic!(format!("{:?} is not valid for Option<T>", index)),
		}
	}
}

impl IndexedStringValue for CompilerVersion {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			unimplemented!();
		}

		let index = indeces.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "version" => indexed_string_value(&self.version, indeces),
			Index::Field(ref id) if id == "commit_hash" => indexed_string_value(&self.commit_hash, indeces),
			Index::Field(ref id) if id == "commit_date" => indexed_string_value(&self.commit_date, indeces),
			Index::Field(ref id) if id == "channel" => indexed_string_value(&self.channel, indeces),
			Index::Field(ref id) if id == "host_triple" => indexed_string_value(&self.host_triple, indeces),
			Index::Field(ref id) if id == "target_triple" => indexed_string_value(&self.target_triple, indeces),
			_ => panic!(format!("{:?} is not valid for versionator::CompilerVersion", index)),
		}
	}
}

impl IndexedStringValue for Version {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			return self.to_string();
		}

		let index = indeces.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "major" => indexed_string_value(&self.major, indeces),
			Index::Field(ref id) if id == "minor" => indexed_string_value(&self.minor, indeces),
			Index::Field(ref id) if id == "patch" => indexed_string_value(&self.patch, indeces),
			Index::Field(ref id) if id == "pre" => indexed_string_value(&self.pre, indeces),
			Index::Field(ref id) if id == "build" => indexed_string_value(&self.build, indeces),
			_ => panic!(format!("{:?} is not valid for versionator::CompilerVersion", index)),
		}
	}
}

impl IndexedStringValue for bool {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			return format!("{}", self);
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("{:?} is not valid for bool", index));
	}
}

impl IndexedStringValue for u64 {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			return format!("{}", self);
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("{:?} is not valid for u64", index));
	}
}

impl IndexedStringValue for usize {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			return format!("{}", self);
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("{:?} is not valid for usize", index));
	}
}

impl IndexedStringValue for String {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			return self.to_string();
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("{:?} is not valid for String", index));
	}
}

impl<T: IndexedStringValue + std::fmt::Display> IndexedStringValue for Vec<T> {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			return self.iter().enumerate().fold(String::new(), |mut acc, (index, value)| {
				&if index == self.len() - 1 {
					format!(acc, "{}", value)
				} else {
					format!(acc, "{}, ", value)
				};
				acc
			});
		}

		let index = indeces.pop_front().unwrap();
		match index {
			Index::Function(ref id, ref args) if id == "len" && args.is_empty() => indexed_string_value(&self.len(), indeces),
			_ => panic!(format!("{:?} is not valid for Vec<T: Display>", index)),
		}
	}
}

impl IndexedStringValue for Identifier {
	fn indexed_string_value(&self, indeces: VecDeque<Index>) -> String {
		match self {
			Identifier::Numeric(value) => indexed_string_value(value, indeces),
			Identifier::AlphaNumeric(value) => indexed_string_value(value, indeces),
		}
	}
}

impl IndexedStringValue for CompilerChannel {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			return self.to_string();
		}

		let index = indeces.pop_front().unwrap();
		panic!(format!("{:?} is not valid for versionator::CompilerChannel", index));
	}
}

impl IndexedStringValue for VersionControl {
	fn indexed_string_value(&self, indeces: VecDeque<Index>) -> String {
		match self {
			versionator_common::VersionControl::Git(value) => indexed_string_value(value, indeces),
		}
	}
}

impl IndexedStringValue for GitInformation {
	fn indexed_string_value(&self, mut indeces: VecDeque<Index>) -> String {
		if indeces.is_empty() {
			let dirty = if self.dirty { "+" } else { "" };
			if let Some(name) = &self.name {
				return format!("{}{} ({})", &self.commit_hash, dirty, name);
			} else {
				return format!("{}{}", &self.commit_hash, dirty);
			}
		}

		let index = indeces.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "commit_hash" => indexed_string_value(&self.commit_hash, indeces),
			Index::Field(ref id) if id == "dirty" => indexed_string_value(&self.dirty, indeces),
			Index::Field(ref id) if id == "name" => indexed_string_value(&self.name, indeces),
			_ => panic!(format!("{:?} is not valid for versionator::GitInformation", index)),
		}
	}
}
