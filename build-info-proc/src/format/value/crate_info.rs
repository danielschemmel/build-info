use anyhow::Result;
use format_buf::format;

use std::any::Any;

use build_info_common::CrateInfo;

use super::{as_arguments_0, as_field_name, FormatSpecifier, Type, Value, OP_FIELD_ACCESS};

impl Value for CrateInfo {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> Result<Box<dyn Value>> {
		match func {
			OP_FIELD_ACCESS => match as_field_name(args) {
				"name" => Ok(Box::new(self.name.clone())),
				"version" => Ok(Box::new(self.version.clone())),
				"authors" => Ok(Box::new(self.authors.clone())),
				"license" => Ok(Box::new(self.license.clone())),
				"enabled_features" => Ok(Box::new(self.enabled_features.clone())),
				"available_features" => Ok(Box::new(self.available_features.clone())),
				"dependencies" => Ok(Box::new(self.dependencies.clone())),
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
