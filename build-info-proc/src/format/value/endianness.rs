use std::any::Any;

use build_info_common::Endianness;

use super::{as_arguments_0, FormatSpecifier, Type, Value};

impl Value for Endianness {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
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
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => write!(buffer, "{self}").unwrap(),
			FormatSpecifier::Debug => write!(buffer, "{self:?}").unwrap(),
			FormatSpecifier::DebugAlt => write!(buffer, "{self:#?}").unwrap(),
		}
	}
}
