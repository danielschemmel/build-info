use anyhow::Result;
use format_buf::format;

use std::any::Any;

use build_info_common::VersionControl;

use super::{as_arguments_0, FormatSpecifier, Type, Value};

impl Value for VersionControl {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> Result<Box<dyn Value>> {
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
			FormatSpecifier::Default => format!(buffer, "{self}"),
			FormatSpecifier::Debug => format!(buffer, "{self:?}"),
			FormatSpecifier::DebugAlt => format!(buffer, "{self:#?}"),
		}
	}
}
