use format_buf::format;

use build_info_common::chrono::{DateTime, NaiveDate, Utc};
use build_info_common::semver::{Identifier, Version};
use build_info_common::{BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, GitInfo, VersionControl};

use super::Index;

pub(crate) fn indexed_string_value<T: IndexedStringValue>(this: &T, indices: &[Index]) -> String {
	this.indexed_string_value(indices)
}

pub(crate) trait IndexedStringValue {
	fn indexed_string_value(&self, indices: &[Index]) -> String;
}

impl IndexedStringValue for BuildInfo {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = &indices[0];
		match index {
			Index::Field(ref id) if id == "timestamp" => indexed_string_value(&self.timestamp, &indices[1..]),
			Index::Field(ref id) if id == "profile" => indexed_string_value(&self.profile, &indices[1..]),
			Index::Field(ref id) if id == "crate_info" => indexed_string_value(&self.crate_info, &indices[1..]),
			Index::Field(ref id) if id == "compiler" => indexed_string_value(&self.compiler, &indices[1..]),
			Index::Field(ref id) if id == "version_control" => indexed_string_value(&self.version_control, &indices[1..]),
			_ => panic!(format!("{:?} is not valid on build_info::BuildInfo", index)),
		}
	}
}

impl IndexedStringValue for CrateInfo {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = &indices[0];
		match index {
			Index::Field(ref id) if id == "name" => indexed_string_value(&self.name, &indices[1..]),
			Index::Field(ref id) if id == "version" => indexed_string_value(&self.version, &indices[1..]),
			Index::Field(ref id) if id == "authors" => indexed_string_value(&self.authors, &indices[1..]),
			_ => panic!(format!("{:?} is not valid for build_info::CrateInfo", index)),
		}
	}
}

impl IndexedStringValue for DateTime<Utc> {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.format("%Y-%m-%d %H:%M:%SZ").to_string();
		}

		let index = &indices[0];
		panic!(format!("{:?} is not valid for DateTime<Utc>", index));
	}
}

impl IndexedStringValue for NaiveDate {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.format("%Y-%m-%d").to_string();
		}

		let index = &indices[0];
		panic!(format!("{:?} is not valid for NaiveDate", index));
	}
}

impl<T: IndexedStringValue> IndexedStringValue for Option<T> {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			if let Some(value) = self {
				return indexed_string_value(value, indices);
			} else {
				return "UNKNOWN".to_string();
			}
		}

		let index = &indices[0];
		match index {
			Index::Unwrap => match self {
				Some(value) => indexed_string_value(value, &indices[1..]),
				None => panic!("Failed to unwrap element"),
			},
			_ => panic!(format!("{:?} is not valid for Option<T>", index)),
		}
	}
}

impl IndexedStringValue for CompilerInfo {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = &indices[0];
		match index {
			Index::Field(ref id) if id == "version" => indexed_string_value(&self.version, &indices[1..]),
			Index::Field(ref id) if id == "commit_id" => indexed_string_value(&self.commit_id, &indices[1..]),
			Index::Field(ref id) if id == "commit_date" => indexed_string_value(&self.commit_date, &indices[1..]),
			Index::Field(ref id) if id == "channel" => indexed_string_value(&self.channel, &indices[1..]),
			Index::Field(ref id) if id == "host_triple" => indexed_string_value(&self.host_triple, &indices[1..]),
			Index::Field(ref id) if id == "target_triple" => indexed_string_value(&self.target_triple, &indices[1..]),
			_ => panic!(format!("{:?} is not valid for build_info::CompilerInfo", index)),
		}
	}
}

impl IndexedStringValue for Version {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = &indices[0];
		match index {
			Index::Field(ref id) if id == "major" => indexed_string_value(&self.major, &indices[1..]),
			Index::Field(ref id) if id == "minor" => indexed_string_value(&self.minor, &indices[1..]),
			Index::Field(ref id) if id == "patch" => indexed_string_value(&self.patch, &indices[1..]),
			Index::Field(ref id) if id == "pre" => indexed_string_value(&self.pre, &indices[1..]),
			Index::Field(ref id) if id == "build" => indexed_string_value(&self.build, &indices[1..]),
			_ => panic!(format!("{:?} is not valid for build_info::CompilerInfo", index)),
		}
	}
}

impl IndexedStringValue for bool {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return format!("{}", self);
		}

		let index = &indices[0];
		panic!(format!("{:?} is not valid for bool", index));
	}
}

impl IndexedStringValue for u64 {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return format!("{}", self);
		}

		let index = &indices[0];
		panic!(format!("{:?} is not valid for u64", index));
	}
}

impl IndexedStringValue for usize {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return format!("{}", self);
		}

		let index = &indices[0];
		panic!(format!("{:?} is not valid for usize", index));
	}
}

impl IndexedStringValue for String {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = &indices[0];
		panic!(format!("{:?} is not valid for String", index));
	}
}

impl<T: IndexedStringValue + std::fmt::Display> IndexedStringValue for Vec<T> {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.iter().enumerate().fold(String::new(), |mut acc, (index, value)| {
				if index == self.len() - 1 {
					format!(acc, "{}", value)
				} else {
					format!(acc, "{}, ", value)
				};
				acc
			});
		}

		let index = &indices[0];
		match index {
			Index::Function(ref id, ref args) if id == "len" && args.is_empty() => indexed_string_value(&self.len(), &indices[1..]),
			_ => panic!(format!("{:?} is not valid for Vec<T: Display>", index)),
		}
	}
}

impl IndexedStringValue for Identifier {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		match self {
			Identifier::Numeric(value) => indexed_string_value(value, indices),
			Identifier::AlphaNumeric(value) => indexed_string_value(value, indices),
		}
	}
}

impl IndexedStringValue for CompilerChannel {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = &indices[0];
		panic!(format!("{:?} is not valid for build_info::CompilerChannel", index));
	}
}

impl IndexedStringValue for VersionControl {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		match self {
			build_info_common::VersionControl::Git(value) => indexed_string_value(value, indices),
		}
	}
}

impl IndexedStringValue for GitInfo {
	fn indexed_string_value(&self, indices: &[Index]) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = &indices[0];
		match index {
			Index::Field(ref id) if id == "commit_id" => indexed_string_value(&self.commit_id, &indices[1..]),
			Index::Field(ref id) if id == "commit_short_id" => indexed_string_value(&self.commit_short_id, &indices[1..]),
			Index::Field(ref id) if id == "commit_timestamp" => indexed_string_value(&self.commit_timestamp, &indices[1..]),
			Index::Field(ref id) if id == "dirty" => indexed_string_value(&self.dirty, &indices[1..]),
			Index::Field(ref id) if id == "branch" => indexed_string_value(&self.branch, &indices[1..]),
			Index::Field(ref id) if id == "tags" => indexed_string_value(&self.tags, &indices[1..]),
			_ => panic!(format!("{:?} is not valid for build_info::GitInfo", index)),
		}
	}
}
