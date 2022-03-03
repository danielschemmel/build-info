use anyhow::{anyhow, Result};
use format_buf::format;

use std::any::Any;

use super::{as_arguments_0, as_arguments_1, FormatSpecifier, Type, Value};

impl<T: 'static + Value + Clone> Value for Option<T> {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> Result<Box<dyn Value>> {
		match func {
			"is_none" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.is_none()))
			}
			"is_some" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.is_some()))
			}
			"expect" => {
				let (message,) = as_arguments_1::<String>(args)?;
				self
					.clone()
					.map(|value| Box::new(value) as Box<dyn Value>)
					.ok_or_else(|| {
						anyhow!("Could not unwrap Option (object does not contain a value)").context(message.to_string())
					})
			}
			"unwrap" => {
				as_arguments_0(args)?;
				self
					.clone()
					.map(|value| Box::new(value) as Box<dyn Value>)
					.ok_or_else(|| anyhow!("Could not unwrap Option (object does not contain a value)"))
			}
			"?" => {
				as_arguments_0(args)?;
				self
					.clone()
					.map(|value| Box::new(value) as Box<dyn Value>)
					.ok_or_else(|| anyhow!("Could not unwrap Option (object does not contain a value)"))
			}
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
			FormatSpecifier::Debug => format!(buffer, "{self:?}"),
			FormatSpecifier::DebugAlt => format!(buffer, "{self:#?}"),
		}
	}
}
