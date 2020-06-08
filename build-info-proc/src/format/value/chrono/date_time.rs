use anyhow::Result;
use format_buf::format;

use std::any::Any;

use build_info_common::chrono::{DateTime, Utc};

use super::super::{as_arguments_0, as_arguments_1, FormatSpecifier, Type, Value};

impl Value for DateTime<Utc> {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"format" => {
				let (format_string,) = as_arguments_1::<String>(args)?;
				Ok(Box::new(self.format(&format_string).to_string()))
			}
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.format("%Y-%m-%d %H:%M:%SZ").to_string()))
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
			FormatSpecifier::Default => format!(buffer, "{}", self.format("%Y-%m-%d %H:%M:%SZ")),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}
