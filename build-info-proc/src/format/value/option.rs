use std::any::Any;

use anyhow::anyhow;

use super::{as_arguments_0, as_simple_arguments_1, FormatSpecifier, Type, Value};

impl<T: 'static + Value + Clone> Value for Option<T> {
	fn call(&self, func: &str, args: &[Box<dyn Value>]) -> anyhow::Result<Box<dyn Value>> {
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
				let (message,) = as_simple_arguments_1::<String>(args)?;
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
		use std::fmt::Write;

		match spec {
			FormatSpecifier::Default => match self {
				Some(value) => value.format(buffer, spec),
				None => *buffer += "None",
			},
			FormatSpecifier::Debug => write!(buffer, "{self:?}").unwrap(),
			FormatSpecifier::DebugAlt => write!(buffer, "{self:#?}").unwrap(),
		}
	}
}

#[cfg(test)]
mod test {
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn format_default() {
		let mut buff = String::new();
		Value::format(&Some(true), &mut buff, FormatSpecifier::Default);
		assert_eq!(buff, "true");

		buff.clear();
		Value::format(&None as &Option<bool>, &mut buff, FormatSpecifier::Default);
		assert_eq!(buff, "None");
	}

	#[test]
	fn format_debug() {
		let mut buff = String::new();
		Value::format(&Some(true), &mut buff, FormatSpecifier::Debug);
		assert_eq!(buff, "Some(true)");

		buff.clear();
		Value::format(&None as &Option<bool>, &mut buff, FormatSpecifier::Debug);
		assert_eq!(buff, "None");
	}

	#[test]
	fn format_debug_alt() {
		let mut buff = String::new();
		Value::format(&Some(true), &mut buff, FormatSpecifier::DebugAlt);
		assert_eq!(buff, "Some(\n    true,\n)");

		buff.clear();
		Value::format(&None as &Option<bool>, &mut buff, FormatSpecifier::DebugAlt);
		assert_eq!(buff, "None");
	}
}
