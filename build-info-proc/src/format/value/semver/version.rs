use std::any::Any;

use build_info_common::semver::Version;
use num_bigint::BigInt;

use super::super::{FormatSpecifier, OP_FIELD_ACCESS, Type, Value, as_arguments_0, as_field_name};

impl Value for Version {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
		match func {
			OP_FIELD_ACCESS => match as_field_name(args) {
				"major" => Ok(Box::new(BigInt::from(self.major))),
				"minor" => Ok(Box::new(BigInt::from(self.minor))),
				"patch" => Ok(Box::new(BigInt::from(self.patch))),
				"pre" => Ok(Box::new(self.pre.to_string())),
				"build" => Ok(Box::new(self.build.to_string())),
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
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => write!(buffer, "{self}").unwrap(),
			FormatSpecifier::Debug => write!(buffer, "{self:?}").unwrap(),
			FormatSpecifier::DebugAlt => write!(buffer, "{self:#?}").unwrap(),
		}
	}
}
