use std::any::Any;

use build_info_common::CompilerInfo;

use super::{FormatSpecifier, OP_FIELD_ACCESS, Type, Value, as_arguments_0, as_field_name};

impl Value for CompilerInfo {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
		match func {
			OP_FIELD_ACCESS => match as_field_name(args) {
				"version" => Ok(Box::new(self.version.clone())),
				"commit_id" => Ok(Box::new(self.commit_id.clone())),
				"commit_date" => Ok(Box::new(self.commit_date)),
				"channel" => Ok(Box::new(self.channel)),
				"host_triple" => Ok(Box::new(self.host_triple.clone())),
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
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => write!(buffer, "{self}").unwrap(),
			FormatSpecifier::Debug => write!(buffer, "{self:?}").unwrap(),
			FormatSpecifier::DebugAlt => write!(buffer, "{self:#?}").unwrap(),
		}
	}
}
