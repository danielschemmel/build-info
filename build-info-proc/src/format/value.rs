use anyhow::{anyhow, Result};
use format_buf::format;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

use std::any::{type_name, Any};
use std::fmt::Debug;

use build_info_common::chrono::{DateTime, NaiveDate, Utc};
use build_info_common::semver::{Identifier, Version};
use build_info_common::{BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, GitInfo, VersionControl};

use super::Type;

fn as_field_name<'a>(args: &[&'a dyn Value]) -> &'a str {
	assert!(
		args.len() == 1,
		"Accessing a field must have exactly one operand (the field name)"
	);

	args[0]
		.as_any()
		.downcast_ref::<String>()
		.expect("The field name must be a string when accessing a field.")
}

fn as_index(args: &[&dyn Value]) -> usize {
	assert!(
		args.len() == 1,
		"Accessing a field must have exactly one operand (the field name)"
	);

	args[0]
		.as_any()
		.downcast_ref::<BigInt>()
		.expect("The array index must be an integer.")
		.to_usize()
		.expect("The array index does not fit into the type usize.")
}

fn as_arguments_0(args: &[&dyn Value]) -> Result<()> {
	if args.is_empty() {
		Ok(())
	} else {
		Err(anyhow!("Wrong number of arguments (should be 0)"))
	}
}

#[allow(dead_code)]
fn as_arguments_1<'a, T1: 'static>(args: &[&'a dyn Value]) -> Result<(&'a T1,)> {
	if args.len() != 1 {
		return Err(anyhow!("Wrong number of arguments (should be 1)"));
	}

	Ok((args[0]
		.as_any()
		.downcast_ref::<T1>()
		.ok_or_else(|| anyhow!("Argument #1 should have type {}", type_name::<T1>()))?,))
}

#[allow(dead_code)]
fn as_arguments_2<'a, T1: 'static, T2: 'static>(args: &[&'a dyn Value]) -> Result<(&'a T1, &'a T2)> {
	if args.len() != 2 {
		return Err(anyhow!("Wrong number of arguments (should be 1)"));
	}

	Ok((
		args[0]
			.as_any()
			.downcast_ref::<T1>()
			.ok_or_else(|| anyhow!("Argument #1 should have type {}", type_name::<T1>()))?,
		args[1]
			.as_any()
			.downcast_ref::<T2>()
			.ok_or_else(|| anyhow!("Argument #2 should have type {}", type_name::<T1>()))?,
	))
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum FormatSpecifier {
	Default,
	Debug,
	DebugAlt,
}

pub(crate) trait Value: Debug {
	fn call_base(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"!field" => {
				let field = as_field_name(args);
				Err(anyhow!(
					"The field {} does not exist for objects of type {}",
					field,
					self.get_type()
				))
			}
			"!tuple_index" => Err(anyhow!("Type {} cannot be tuple-indexed", self.get_type())),
			"::std::ops::Index::index" => Err(anyhow!("Type {} cannot be indexed", self.get_type())),
			_ => Err(anyhow!(
				"Function {} cannot be called with arguments {:#?} on objects of type {}",
				func,
				args,
				self.get_type()
			)),
		}
	}

	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		self.call_base(func, args)
	}

	fn get_type(&self) -> Type;

	fn as_any(&self) -> &dyn Any;

	fn format(&self, buffer: &mut String, spec: FormatSpecifier);
}

impl Value for bool {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Bool
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for BigInt {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Integer
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for String {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::String
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => *buffer += self,
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl<T: 'static + Value + Clone> Value for Option<T> {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"?" => self
				.clone()
				.map(|value| Box::new(value) as Box<dyn Value>)
				.ok_or_else(|| anyhow!("Could not unwrap Option (object does not contain a value)")),
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Option
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => match self {
				Some(value) => value.format(buffer, spec),
				None => *buffer += "None",
			},
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl<T: 'static + Value + Clone> Value for Vec<T> {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"::std::ops::Index::index" => Ok(Box::new(self[as_index(args)].clone())),
			"len" => {
				as_arguments_0(args)?;
				Ok(Box::new(BigInt::from(self.len())))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Option
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => {
				for (i, value) in self.iter().enumerate() {
					if i > 0 {
						if i < self.len() - 1 {
							*buffer += ", ";
						} else {
							*buffer += " and ";
						}
					}
					value.format(buffer, spec);
				}
			}
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for BuildInfo {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"!field" => match as_field_name(args) {
				"timestamp" => Ok(Box::new(self.timestamp)),
				"profile" => Ok(Box::new(self.profile.clone())),
				"crate_info" => Ok(Box::new(self.crate_info.clone())),
				"compiler" => Ok(Box::new(self.compiler.clone())),
				"version_control" => Ok(Box::new(self.version_control.clone())),
				_ => self.call_base(func, args),
			},
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::BuildInfo
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for CrateInfo {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"!field" => match as_field_name(args) {
				"name" => Ok(Box::new(self.name.clone())),
				"version" => Ok(Box::new(self.version.clone())),
				"authors" => Ok(Box::new(self.authors.clone())),
				_ => self.call_base(func, args),
			},
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::CrateInfo
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for CompilerInfo {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"!field" => match as_field_name(args) {
				"version" => Ok(Box::new(self.version.clone())),
				"commit_id" => Ok(Box::new(self.commit_id.clone())),
				"commit_date" => Ok(Box::new(self.commit_date)),
				"channel" => Ok(Box::new(self.channel)),
				"host_triple" => Ok(Box::new(self.host_triple.clone())),
				"target_triple" => Ok(Box::new(self.target_triple.clone())),
				_ => self.call_base(func, args),
			},
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::CompilerInfo
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for CompilerChannel {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::CompilerChannel
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for VersionControl {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"git" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.git().cloned()))
			}
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::VersionControl
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for GitInfo {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"!field" => match as_field_name(args) {
				"commit_id" => Ok(Box::new(self.commit_id.clone())),
				"commit_short_id" => Ok(Box::new(self.commit_short_id.clone())),
				"commit_timestamp" => Ok(Box::new(self.commit_timestamp)),
				"dirty" => Ok(Box::new(self.dirty)),
				"branch" => Ok(Box::new(self.branch.clone())),
				"tags" => Ok(Box::new(self.tags.clone())),
				_ => self.call_base(func, args),
			},
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::GitInfo
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for DateTime<Utc> {
	fn get_type(&self) -> Type {
		Type::DateTimeUtc
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self.format("%Y-%m-%d %H:%M:%SZ")),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for NaiveDate {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.format("%Y-%m-%d").to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::DateTimeUtc
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for Version {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"!field" => match as_field_name(args) {
				"major" => Ok(Box::new(BigInt::from(self.major))),
				"minor" => Ok(Box::new(BigInt::from(self.minor))),
				"patch" => Ok(Box::new(BigInt::from(self.patch))),
				"pre" => Ok(Box::new(self.pre.clone())),
				"build" => Ok(Box::new(self.build.clone())),
				_ => self.call_base(func, args),
			},
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Version
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}

impl Value for Identifier {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::Version
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}
