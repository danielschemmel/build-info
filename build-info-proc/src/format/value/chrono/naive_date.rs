use anyhow::Result;
use format_buf::format;

use std::any::Any;

use build_info_common::chrono::NaiveDate;

use super::super::{as_arguments_0, as_arguments_1, FormatSpecifier, Type, Value};

impl Value for NaiveDate {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> Result<Box<dyn Value>> {
		match func {
			"format" => {
				let (format_string,) = as_arguments_1::<String>(args)?;
				Ok(Box::new(self.format(&format_string).to_string()))
			}
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
