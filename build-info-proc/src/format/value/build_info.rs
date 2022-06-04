use std::any::Any;

use build_info_common::BuildInfo;

use super::{as_arguments_0, as_field_name, FormatSpecifier, Type, Value, OP_FIELD_ACCESS};

impl Value for BuildInfo {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
		match func {
			OP_FIELD_ACCESS => match as_field_name(args) {
				"timestamp" => Ok(Box::new(self.timestamp)),
				"profile" => Ok(Box::new(self.profile.clone())),
				"optimization_level" => Ok(Box::new(self.optimization_level)),
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
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => write!(buffer, "{self}").unwrap(),
			FormatSpecifier::Debug => write!(buffer, "{self:?}").unwrap(),
			FormatSpecifier::DebugAlt => write!(buffer, "{self:#?}").unwrap(),
		}
	}
}
