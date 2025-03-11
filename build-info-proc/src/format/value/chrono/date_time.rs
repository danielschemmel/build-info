use std::any::Any;

use chrono::{DateTime, Utc};

use super::super::{FormatSpecifier, Type, Value, as_arguments_0, as_simple_arguments_1};

impl Value for DateTime<Utc> {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
		match func {
			"format" => {
				let (format_string,) = as_simple_arguments_1::<String>(args)?;
				Ok(Box::new(self.format(format_string).to_string()))
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
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => write!(buffer, "{}", self.format("%Y-%m-%d %H:%M:%SZ")).unwrap(),
			FormatSpecifier::Debug => write!(buffer, "{self:?}").unwrap(),
			FormatSpecifier::DebugAlt => write!(buffer, "{self:#?}").unwrap(),
		}
	}
}
