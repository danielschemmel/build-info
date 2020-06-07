use format_buf::format;

use std::collections::VecDeque;

use build_info_common::{
	BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, DateTime, GitInfo, Identifier, Utc, Version, VersionControl,
};

use super::Index;

pub(crate) fn indexed_string_value<T: IndexedStringValue>(this: &T, indices: VecDeque<Index>) -> String {
	this.indexed_string_value(indices)
}

pub(crate) trait IndexedStringValue {
	fn indexed_string_value(&self, indices: VecDeque<Index>) -> String;
}

impl IndexedStringValue for BuildInfo {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = indices.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "timestamp" => indexed_string_value(&self.timestamp, indices),
			Index::Field(ref id) if id == "profile" => indexed_string_value(&self.profile, indices),
			Index::Field(ref id) if id == "crate_info" => indexed_string_value(&self.crate_info, indices),
			Index::Field(ref id) if id == "compiler" => indexed_string_value(&self.compiler, indices),
			Index::Field(ref id) if id == "version_control" => indexed_string_value(&self.version_control, indices),
			_ => panic!(format!("{:?} is not valid on build_info::BuildInfo", index)),
		}
	}
}

impl IndexedStringValue for CrateInfo {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = indices.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "name" => indexed_string_value(&self.name, indices),
			Index::Field(ref id) if id == "version" => indexed_string_value(&self.version, indices),
			Index::Field(ref id) if id == "authors" => indexed_string_value(&self.authors, indices),
			_ => panic!(format!("{:?} is not valid for build_info::CrateInfo", index)),
		}
	}
}

impl IndexedStringValue for DateTime<Utc> {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return self.format("%Y-%m-%d %H:%M:%SZ").to_string();
		}

		let index = indices.pop_front().unwrap();
		panic!(format!("{:?} is not valid for DateTime<Utc>", index));
	}
}

impl<T: IndexedStringValue> IndexedStringValue for Option<T> {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			if let Some(value) = self {
				return indexed_string_value(value, indices);
			} else {
				return "UNKNOWN".to_string();
			}
		}

		let index = indices.pop_front().unwrap();
		match index {
			Index::Unwrap => match self {
				Some(value) => indexed_string_value(value, indices),
				None => panic!("Failed to unwrap element"),
			},
			_ => panic!(format!("{:?} is not valid for Option<T>", index)),
		}
	}
}

impl IndexedStringValue for CompilerInfo {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = indices.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "version" => indexed_string_value(&self.version, indices),
			Index::Field(ref id) if id == "commit_id" => indexed_string_value(&self.commit_id, indices),
			Index::Field(ref id) if id == "commit_date" => indexed_string_value(&self.commit_date, indices),
			Index::Field(ref id) if id == "channel" => indexed_string_value(&self.channel, indices),
			Index::Field(ref id) if id == "host_triple" => indexed_string_value(&self.host_triple, indices),
			Index::Field(ref id) if id == "target_triple" => indexed_string_value(&self.target_triple, indices),
			_ => panic!(format!("{:?} is not valid for build_info::CompilerInfo", index)),
		}
	}
}

impl IndexedStringValue for Version {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = indices.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "major" => indexed_string_value(&self.major, indices),
			Index::Field(ref id) if id == "minor" => indexed_string_value(&self.minor, indices),
			Index::Field(ref id) if id == "patch" => indexed_string_value(&self.patch, indices),
			Index::Field(ref id) if id == "pre" => indexed_string_value(&self.pre, indices),
			Index::Field(ref id) if id == "build" => indexed_string_value(&self.build, indices),
			_ => panic!(format!("{:?} is not valid for build_info::CompilerInfo", index)),
		}
	}
}

impl IndexedStringValue for bool {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return format!("{}", self);
		}

		let index = indices.pop_front().unwrap();
		panic!(format!("{:?} is not valid for bool", index));
	}
}

impl IndexedStringValue for u64 {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return format!("{}", self);
		}

		let index = indices.pop_front().unwrap();
		panic!(format!("{:?} is not valid for u64", index));
	}
}

impl IndexedStringValue for usize {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return format!("{}", self);
		}

		let index = indices.pop_front().unwrap();
		panic!(format!("{:?} is not valid for usize", index));
	}
}

impl IndexedStringValue for String {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = indices.pop_front().unwrap();
		panic!(format!("{:?} is not valid for String", index));
	}
}

impl<T: IndexedStringValue + std::fmt::Display> IndexedStringValue for Vec<T> {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
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

		let index = indices.pop_front().unwrap();
		match index {
			Index::Function(ref id, ref args) if id == "len" && args.is_empty() => indexed_string_value(&self.len(), indices),
			_ => panic!(format!("{:?} is not valid for Vec<T: Display>", index)),
		}
	}
}

impl IndexedStringValue for Identifier {
	fn indexed_string_value(&self, indices: VecDeque<Index>) -> String {
		match self {
			Identifier::Numeric(value) => indexed_string_value(value, indices),
			Identifier::AlphaNumeric(value) => indexed_string_value(value, indices),
		}
	}
}

impl IndexedStringValue for CompilerChannel {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = indices.pop_front().unwrap();
		panic!(format!("{:?} is not valid for build_info::CompilerChannel", index));
	}
}

impl IndexedStringValue for VersionControl {
	fn indexed_string_value(&self, indices: VecDeque<Index>) -> String {
		match self {
			build_info_common::VersionControl::Git(value) => indexed_string_value(value, indices),
		}
	}
}

impl IndexedStringValue for GitInfo {
	fn indexed_string_value(&self, mut indices: VecDeque<Index>) -> String {
		if indices.is_empty() {
			return self.to_string();
		}

		let index = indices.pop_front().unwrap();
		match index {
			Index::Field(ref id) if id == "commit_id" => indexed_string_value(&self.commit_id, indices),
			Index::Field(ref id) if id == "commit_short_id" => indexed_string_value(&self.commit_short_id, indices),
			Index::Field(ref id) if id == "commit_timestamp" => indexed_string_value(&self.commit_timestamp, indices),
			Index::Field(ref id) if id == "dirty" => indexed_string_value(&self.dirty, indices),
			Index::Field(ref id) if id == "branch" => indexed_string_value(&self.branch, indices),
			Index::Field(ref id) if id == "tags" => indexed_string_value(&self.tags, indices),
			_ => panic!(format!("{:?} is not valid for build_info::GitInfo", index)),
		}
	}
}
