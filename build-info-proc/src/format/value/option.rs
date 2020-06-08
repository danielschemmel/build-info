use anyhow::{anyhow, Result};
use format_buf::format;

use std::any::Any;

use super::{FormatSpecifier, Type, Value};

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
